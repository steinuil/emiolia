use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FileType {
    Pdf = 0,
}

#[derive(Debug)]
pub struct Document {
    id: DocumentId,
    sha256: [u8; 64],
    file_type: FileType,
    page_count: u16,
    import_filename: Option<String>,
    imported_at: Instant,
}
