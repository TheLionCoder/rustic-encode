mod data_loading;
mod encoding;
mod file_extension;

fn main() {
    encoding::process_files().unwrap();
}
