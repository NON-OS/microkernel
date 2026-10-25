// NONOS Operating System (AGPL-3.0-or-later)

//! A sidecar file read whole or refused. The read asks for one byte past
//! the most a sidecar holds, and a file that fills it is not taken, so a
//! record cut mid-path is never loaded and then saved back cut.

use alloc::vec::Vec;

/// Why a sidecar was not taken: it is larger than one can be.
pub const TOO_LONG: &str = "sidecar larger than a sidecar holds";

/// What to ask a read for when at most `max` bytes are taken.
pub const fn ask(max: u32) -> u32 {
    max.saturating_add(1)
}

/// `read`, made with [`ask`], when it is the whole file; a file past `max`
/// is refused with [`TOO_LONG`], and a failed read stays as it failed.
pub fn whole(read: Result<Vec<u8>, &'static str>, max: u32) -> Result<Vec<u8>, &'static str> {
    let bytes = read?;
    if bytes.len() as u64 > u64::from(max) {
        return Err(TOO_LONG);
    }
    Ok(bytes)
}
