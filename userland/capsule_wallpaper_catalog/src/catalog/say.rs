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

//! Why a wallpaper could not be served, on the serial line. The wallpaper
//! service asks again every few seconds, so each wallpaper's reason is said
//! once; a silent failure here left the desktop on its built in picture with
//! nothing anywhere saying which step broke.

use core::sync::atomic::{AtomicU64, Ordering};

/// The wallpapers whose reason has been said, one bit per index.
static SAID: AtomicU64 = AtomicU64::new(0);

/// Say "[WALLPAPER-CATALOG] <slug>: <why>", the first time for `index`.
pub fn refused(index: u32, slug: &[u8], why: &str) {
    let bit = 1u64.checked_shl(index).unwrap_or(0);
    if bit != 0 && SAID.fetch_or(bit, Ordering::Relaxed) & bit != 0 {
        return;
    }
    let mut line = alloc::vec::Vec::with_capacity(64);
    line.extend_from_slice(b"[WALLPAPER-CATALOG] ");
    line.extend_from_slice(slug);
    line.extend_from_slice(b": ");
    line.extend_from_slice(why.as_bytes());
    line.push(b'\n');
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
