use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::DirEntry;

use crate::file_extension::FileExtension;

/// Read a CSV file and return a Vec of the rows.
pub(crate) fn read_file(file_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data_input: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(data_input);

    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

/// Check if a file is hidden
pub(crate) fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Check if a file is a plain text file
pub(crate) fn is_text_or_csv_file(path: &Path) -> bool {
    matches!(
        FileExtension::from_path(path),
        FileExtension::Txt | FileExtension::Csv
    )
}

/// Extract the file name from a path
pub(crate) fn extract_file_stem(path: &Path) -> Result<&str, std::io::Error> {
    Ok(path.file_stem().unwrap().to_str().unwrap())
}
