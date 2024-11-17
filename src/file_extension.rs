use std::path::Path;

pub(crate) enum FileExtension {
    Txt,
    Csv,
    Other
}

impl FileExtension {
    pub(crate) fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("txt") => FileExtension::Txt,
            Some(ext) if ext.eq_ignore_ascii_case("csv") => FileExtension::Csv,
            _ => FileExtension::Other
        }
    }
}