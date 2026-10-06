extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const THUMB_W: u32 = 160;
pub const THUMB_H: u32 = 120;

pub struct Entry {
    pub path: String,
    pub thumb: Option<Vec<u32>>,
    pub tw: u32,
    pub th: u32,
    pub failed: bool,
}

/// Every image the store holds is an entry and a tile; there is no cap on the
/// list. Only the thumbnails near the view are held (`layout::kept`).
pub struct GalleryState {
    pub entries: Vec<Entry>,
    pub scroll: usize,
    pub sel: usize,
    pub scanned: bool,
    /// Why the store could not be listed, when it could not: the gallery says
    /// so instead of "No images found".
    pub scan_error: Option<&'static str>,
    /// A thumbnail failure that is the decoder's, not one file's (it is not
    /// running, or did not answer); said once over the red tiles.
    pub decoder_trouble: Option<&'static str>,
    /// Uptime before which no thumbnail is read: the store did not answer
    /// the last read (says.rs THUMB_QUIET_MS).
    pub quiet_until_ms: u64,
}

impl GalleryState {
    pub fn new() -> Self {
        GalleryState {
            entries: Vec::new(),
            scroll: 0,
            sel: 0,
            scanned: false,
            scan_error: None,
            decoder_trouble: None,
            quiet_until_ms: 0,
        }
    }
}
