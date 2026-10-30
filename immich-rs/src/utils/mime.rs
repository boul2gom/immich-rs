use std::path::Path;

/// Detect the MIME type of a file based on its extension, including modern photography and RAW formats.
pub(crate) fn get_mime_type_for_file<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext.to_ascii_lowercase().as_str() {
            "heic" => return "image/heic".to_string(),
            "heif" => return "image/heif".to_string(),
            "avif" => return "image/avif".to_string(),
            "cr2" => return "image/x-canon-cr2".to_string(),
            "cr3" => return "image/x-canon-cr3".to_string(),
            "nef" => return "image/x-nikon-nef".to_string(),
            "arw" => return "image/x-sony-arw".to_string(),
            "dng" => return "image/x-adobe-dng".to_string(),
            "orf" => return "image/x-olympus-orf".to_string(),
            "rw2" => return "image/x-panasonic-rw2".to_string(),
            "raf" => return "image/x-fuji-raf".to_string(),
            "pef" => return "image/x-pentax-pef".to_string(),
            _ => {}
        }
    }
    mime_guess::from_path(path).first_or_octet_stream().to_string()
}

#[cfg(test)]
#[path = "mime_tests.rs"]
mod tests;
