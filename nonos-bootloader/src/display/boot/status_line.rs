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

use super::layout::splash;
use crate::display::fx::clear_region;
use crate::display::gop::is_initialized;
use crate::display::ink::palette::{CYAN, TEXT_3, WARN};
use crate::display::ink::{label, label_width, metrics, Style};

/// The three facts the splash keeps in view, as mono label and value pairs.
/// The kernel's STARK reads verified only once the loader has checked it;
/// before that it says so instead of claiming it. A development loader checks
/// the kernel's Merkle path alone, and says that, never STARK.
#[cfg(not(feature = "dev-attest"))]
const KERNEL: &[u8] = b"KERNEL STARK";
#[cfg(not(feature = "dev-attest"))]
const VERIFIED: &[u8] = b"VERIFIED";
#[cfg(feature = "dev-attest")]
const KERNEL: &[u8] = b"KERNEL PATH";
#[cfg(feature = "dev-attest")]
const VERIFIED: &[u8] = b"DEV, NO STARK";

pub fn draw_status_line(secure_boot: bool, measured: bool, attested: bool) {
    if !is_initialized() {
        return;
    }
    let s = splash();
    let facts: [(&[u8], &[u8], u32); 3] = [
        (b"SECURE BOOT", if secure_boot { b"ON" } else { b"OFF" }, if secure_boot { CYAN } else { WARN }),
        (b"TPM 2.0", if measured { b"MEASURING" } else { b"NOT FOUND" }, if measured { CYAN } else { WARN }),
        (KERNEL, if attested { VERIFIED } else { b"NOT YET CHECKED" }, if attested { CYAN } else { TEXT_3 }),
    ];
    let line = metrics(Style::Mono).line + 2 * s.u;
    clear_region(s.col_x, s.chips_y, s.col_w, line * 2);
    let half = s.col_w / 2;
    for (i, (name, value, color)) in facts.iter().enumerate() {
        let (x, y) = (s.col_x + (i as u32 % 2) * half, s.chips_y + (i as u32 / 2) * line);
        label(x, y, name, TEXT_3);
        label(x + label_width(KERNEL) + 3 * s.u, y, value, *color);
    }
}
