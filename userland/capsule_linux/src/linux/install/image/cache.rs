//! The chunks of the images fetched so far, kept by the image's pin under
//! /nonos/linux/image-cache, outside every tree a guest writes. A chunk is
//! read back only when it is still its digest; one that is not is dropped
//! and fetched again. An install that stops keeps what it fetched, so the
//! next one asks only for the rest, and an install of an image already here
//! asks for nothing.

use alloc::vec::Vec;

use nonos_app_image::ChunkList;
use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::named::hex;

const DIR: &[u8] = b"/nonos/linux/image-cache";
/// The longest list: one row per 64 KiB chunk of a 1 GiB image.
const LIST_MAX: u32 = 32 + 32 * 16384;

fn dir(pin: &[u8; 32]) -> Vec<u8> {
    [DIR, b"/", hex(pin).as_bytes()].concat()
}

fn at(pin: &[u8; 32], leaf: &str) -> Vec<u8> {
    [dir(pin).as_slice(), b"/", leaf.as_bytes()].concat()
}

/// The list kept for `pin`, if it is still the list `pin` names.
pub fn list(pin: &[u8; 32]) -> Option<Vec<u8>> {
    let raw = vfs::read_file(mk_getpid(), &at(pin, "list"), LIST_MAX).ok()?;
    ChunkList::parse(&raw, pin).is_ok().then_some(raw)
}

/// Keep `raw`, the list `pin` names. A list that cannot be kept costs only
/// a fetch next time.
pub fn keep_list(pin: &[u8; 32], raw: &[u8]) {
    let pid = mk_getpid();
    let _ = vfs::mkdir(pid, DIR);
    let _ = vfs::mkdir(pid, &dir(pin));
    let _ = vfs::write_file(pid, &at(pin, "list"), raw);
}

/// Whether chunk `i` of `pin` is kept, by its length alone.
pub fn present(pin: &[u8; 32], list: &ChunkList, i: usize) -> bool {
    let want = list.range(i).map(|r| r.end - r.start);
    matches!(vfs::stat(mk_getpid(), &at(pin, &leaf(i))), Ok((len, false)) if Some(len) == want)
}

/// Chunk `i`, if it is kept and still its digest; a kept one that is not is
/// dropped.
pub fn chunk(pin: &[u8; 32], list: &ChunkList, i: usize) -> Option<Vec<u8>> {
    let (pid, path) = (mk_getpid(), at(pin, &leaf(i)));
    let raw = vfs::read_file(pid, &path, list.chunk).ok()?;
    if list.holds(i, &raw) {
        return Some(raw);
    }
    let _ = vfs::unlink(pid, &path);
    None
}

/// Keep chunk `i`, already held to its digest. False when the store would
/// not take it: the install goes on with the bytes in hand.
pub fn keep_chunk(pin: &[u8; 32], i: usize, bytes: &[u8]) -> bool {
    vfs::write_file(mk_getpid(), &at(pin, &leaf(i)), bytes).is_ok()
}

fn leaf(i: usize) -> alloc::string::String {
    alloc::format!("{i}")
}
