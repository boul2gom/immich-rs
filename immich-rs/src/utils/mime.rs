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
mod tests {
    use super::*;

    #[test]
    fn mime_detection_resolves_raw_and_modern_photo_formats() {
        assert_eq!(get_mime_type_for_file("photo.heic"), "image/heic");
        assert_eq!(get_mime_type_for_file("photo.HEIF"), "image/heif");
        assert_eq!(get_mime_type_for_file("photo.avif"), "image/avif");
        assert_eq!(get_mime_type_for_file("photo.cr2"), "image/x-canon-cr2");
        assert_eq!(get_mime_type_for_file("photo.CR3"), "image/x-canon-cr3");
        assert_eq!(get_mime_type_for_file("photo.nef"), "image/x-nikon-nef");
        assert_eq!(get_mime_type_for_file("photo.arw"), "image/x-sony-arw");
        assert_eq!(get_mime_type_for_file("photo.dng"), "image/x-adobe-dng");
        assert_eq!(get_mime_type_for_file("photo.orf"), "image/x-olympus-orf");
        assert_eq!(get_mime_type_for_file("photo.rw2"), "image/x-panasonic-rw2");
        assert_eq!(get_mime_type_for_file("photo.raf"), "image/x-fuji-raf");
        assert_eq!(get_mime_type_for_file("photo.pef"), "image/x-pentax-pef");
    }

    #[test]
    fn mime_detection_standard_and_fallback_formats() {
        assert_eq!(get_mime_type_for_file("image.jpg"), "image/jpeg");
        assert_eq!(get_mime_type_for_file("image.png"), "image/png");
        assert_eq!(get_mime_type_for_file("video.mp4"), "video/mp4");
        assert_eq!(get_mime_type_for_file("unknown_no_ext"), "application/octet-stream");
        assert_eq!(
            get_mime_type_for_file("unknown.customext123"),
            "application/octet-stream"
        );
    }
}
