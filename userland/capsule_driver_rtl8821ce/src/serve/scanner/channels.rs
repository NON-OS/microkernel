// NONOS Operating System (AGPL-3.0-or-later)

//! The channels the scan hops through and the frame size it reads into,
//! shared with the join's beacon hunt.

/// The 2.4GHz channels the background scan and the pre-join beacon hunt hop through.
pub(crate) const SCAN_CHANNELS: [u8; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
/// The scratch buffer a scan or a beacon hunt reads each frame into.
pub(crate) const SCAN_FRAME_MAX: usize = 2048;
