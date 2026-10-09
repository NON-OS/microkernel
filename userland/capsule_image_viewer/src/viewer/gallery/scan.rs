extern crate alloc;
use crate::viewer::ext::is_codec_image;
use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::list_paths;

pub fn filter_images(paths: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = paths.into_iter().filter(|p| is_codec_image(p.as_bytes())).collect();
    out.sort();
    out
}

/// The store's images, or why it could not be listed: a failed listing must
/// not read as a store with no images.
pub fn scan(owner_pid: u32) -> Result<Vec<String>, &'static str> {
    list_paths(owner_pid, b"/").map(filter_images)
}
