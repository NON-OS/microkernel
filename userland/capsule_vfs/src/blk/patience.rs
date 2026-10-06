// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! When the boot load stops trying. Pure, so the host proofs hold it.
//!
//! The load gave up after five attempts, two to five seconds after vfs
//! started. A USB stick is found later than that: driver.usb_msc0 looks for
//! its device for up to ten seconds after xHCI is up, and the kernel answers
//! a store read with ETIMEDOUT meanwhile. So a live boot from a stick left
//! the store unread for the whole boot, with no /linux tree and no wallpaper,
//! though the stick held both. A disk that is not there yet, or not ready,
//! is now waited for; a store that is there and wrong is still given up on
//! after a few attempts, since asking again gives the same answer.

use super::error::BlkError;

/// Attempts at a store that answers but does not load.
pub const MAX_ATTEMPTS: u32 = 5;

/// How long after vfs starts a disk that is not there, or not ready, is
/// still looked for: past driver.usb_msc0's ten-second search, which starts
/// only once xHCI is up, and a stick that takes seconds to say it is ready.
/// Thirty seconds was not enough on a laptop booted from a stick.
pub const DISK_WAIT_MS: i64 = 60_000;

/// Whether the error says only that the disk is not there or not ready yet.
pub fn not_yet(e: &BlkError) -> bool {
    matches!(e, BlkError::NoService | BlkError::Transport(_))
}

/// Whether the boot load stops after `attempts`, the last of which failed
/// with `e`, `waited_ms` after vfs started.
pub fn gives_up(e: &BlkError, attempts: u32, waited_ms: i64) -> bool {
    attempts >= MAX_ATTEMPTS && (!not_yet(e) || waited_ms >= DISK_WAIT_MS)
}
