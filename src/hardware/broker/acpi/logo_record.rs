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

//! MkBootLogo's record, which 0 (the syscall's comment says the layout):
//! where firmware drew its logo, what the BMP is, and the firmware mode
//! the logo's place is measured in.

use super::boot_logo::{boot_logo, firmware_mode};

pub const RECORD: usize = 36;
/// The record before the mode was added; a reader asking for this much
/// still gets what it was written for.
pub const OLD_RECORD: usize = 28;

pub fn logo_record() -> Option<[u8; RECORD]> {
    let (b, m) = boot_logo()?;
    let (mode_w, mode_h) = firmware_mode();
    let mut r = [0u8; RECORD];
    for (i, v) in [b.x, b.y, m.width, m.height].iter().enumerate() {
        r[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    r[16] = u8::from(b.displayed) | u8::from(m.top_down) << 1;
    r[17] = b.orientation;
    r[18..20].copy_from_slice(&m.bpp.to_le_bytes());
    for (i, v) in [m.file_size, m.pixels_at, mode_w, mode_h].iter().enumerate() {
        r[20 + 4 * i..24 + 4 * i].copy_from_slice(&v.to_le_bytes());
    }
    Some(r)
}

/// How much of the record a reader with `out_len` bytes is given.
pub fn record_len(out_len: u64) -> usize {
    match out_len {
        l if (OLD_RECORD as u64..RECORD as u64).contains(&l) => OLD_RECORD,
        _ => RECORD,
    }
}
