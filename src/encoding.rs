use encoding_rs::WINDOWS_1252;
use rayon::prelude::*;
use rayon::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use walkdir::WalkDir;

use crate::data_loading::{extract_file_stem, is_hidden, is_text_or_csv_file, read_file};

/// Process all files in the data directory
pub(crate) fn process_files() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir: &Path = Path::new("data/raw");
    let writers: Arc<Mutex<HashMap<String, BufWriter<File>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    WalkDir::new(data_dir)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file() && is_text_or_csv_file(entry.path()))
        .for_each(|entry| {
            let file_path: &Path = entry.path();
            let file_stem: &str = extract_file_stem(file_path).unwrap();

            let data: Vec<u8> = read_file(file_path).unwrap();
            let decoded_data: Vec<String> = decode_data(&data);
            write_records_to_csv(decoded_data, file_stem, writers.clone()).unwrap()
        });

    let mut lock_writers: MutexGuard<HashMap<String, BufWriter<File>>> = writers.lock().unwrap();
    for writer in lock_writers.values_mut() {
        writer.flush().unwrap()
    }

    Ok(())
}

/// Decode data from a file
fn decode_data(data: &[u8]) -> Vec<String> {
    let (decoded, _, _) = WINDOWS_1252.decode(data);
    decoded
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

/// Write records to csv file
fn write_records_to_csv(
    decoded_data: Vec<String>,
    file_stem: &str,
    writers: Arc<Mutex<HashMap<String, BufWriter<File>>>>,
) -> Result<(), csv::Error> {
    let mut lock_writers: MutexGuard<HashMap<String, BufWriter<File>>> = writers.lock().unwrap();
    let writer: &mut BufWriter<File> =
        lock_writers
            .entry(file_stem.to_string())
            .or_insert_with(|| {
                let file: File = File::create(format!("data/processed/{}.csv", file_stem)).unwrap();
                BufWriter::new(file)
            });
    for line in decoded_data {
        if !line.trim().is_empty() {
            writeln!(writer, "{}", line)?;
        }
    }
    writer.flush()?;
    Ok(())
}
