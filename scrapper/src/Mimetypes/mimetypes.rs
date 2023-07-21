use lazy_static::lazy_static;
use std::collections::HashMap;
pub type MimeMap = HashMap<&'static str, &'static str>;
lazy_static! {
    pub static ref MIMETYPES: MimeMap = {
        let mut map = HashMap::new();
        map.insert("txt", "text/plain");
        map.insert(
            "docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        );
        map.insert("doc", "application/msword");
        map.insert("epub", "application/epub+zip");
        map.insert("gif", "image/gif");
        map.insert("png", "image/png");
        map.insert("jpeg", "image/jpeg");
        map.insert("jpg", "image/jpeg");
        map.insert("ppt", "application/vnd.ms-powerpoint");
        map.insert(
            "pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        );
        map.insert("json", "application/json");
        map.insert("pdf", "application/pdf");
        map.insert("mp3", "audio/mpeg");
        map.insert("mpeg", "audio/mpeg");
        map.insert("mp4", "audio/mp4");
        map.insert("rar", "application/vnd.rar");
        map.insert("zip", "application/zip");
        map
    };
}

pub trait Parseable {
    fn parse_mime(&self, input: &str) -> Option<&'static str>;
}
impl Parseable for MimeMap {
    fn parse_mime(&self, input: &str) -> Option<&'static str> {
        MIMETYPES.get(input).copied()
    }
}
