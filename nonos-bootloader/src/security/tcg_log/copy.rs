// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The copy. The firmware names where the log starts and where its last entry
//! starts; the length is read off the log itself, the header and that last
//! entry, by the same parser the kernel replays the log with, viewing only
//! the bytes the structure names.

use alloc::vec::Vec;

use nonos_boot_measure::tcg::{event_walk, grow, spec_id_walk, MAX_LOG_BYTES};
use uefi::table::boot::BootServices;

use super::locate::locate;

/// The log in loader memory the kernel never reclaims, or `None` with no
/// TPM, no log, a truncated one, or one the parser refuses.
pub fn event_log(bs: &BootServices) -> Option<&'static [u8]> {
    let (start, last) = locate(bs)?;
    /*
     * SAFETY: eK@nonos.systems - the firmware owns the log at `start` until
     * ExitBootServices, and `grow` asks only for prefixes the log's own
     * lengths name, never past MAX_LOG_BYTES from either address.
     */
    let view =
        |at: u64| move |n: usize| Some(unsafe { core::slice::from_raw_parts(at as *const u8, n) });
    let (banks, header) = grow(32, MAX_LOG_BYTES, view(start), spec_id_walk).ok()?;
    let len = if last == start {
        header
    } else {
        let off = usize::try_from(last.checked_sub(start)?).ok()?;
        if off < header || off >= MAX_LOG_BYTES {
            return None;
        }
        let ev = grow(12, MAX_LOG_BYTES - off, view(last), |b| event_walk(&banks, b)).ok()?;
        off.checked_add(ev.len)?
    };
    Some(Vec::leak(view(start)(len)?.to_vec()))
}
