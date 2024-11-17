mod encoding;
mod data_loading;
mod file_extension;

fn main() {
    encoding::process_files().unwrap();
}

