use pretty_assertions::assert_eq;

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
fn mime_detection_falls_back_to_octet_stream_for_unknown_extensions() {
    assert_eq!(get_mime_type_for_file("image.jpg"), "image/jpeg");
    assert_eq!(get_mime_type_for_file("image.png"), "image/png");
    assert_eq!(get_mime_type_for_file("video.mp4"), "video/mp4");
    assert_eq!(get_mime_type_for_file("unknown_no_ext"), "application/octet-stream");
    assert_eq!(
        get_mime_type_for_file("unknown.customext123"),
        "application/octet-stream"
    );
}
