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

//! The platform as the loader's checks found it before the menu: four facts
//! as mono label and value pairs, two to a row.

use super::layout::Layout;
use crate::display::ink::palette::{BAD, CYAN, TEXT, TEXT_3, WARN};
use crate::display::ink::{label, label_width, metrics, Style};
use crate::security::{SecurityContext, SecurityPolicy};

pub(super) fn draw_platform(l: &Layout, s: &SecurityContext) {
    let floor: (&[u8], u32) = match SecurityPolicy::from_build() {
        SecurityPolicy::Hardened => (b"HARDENED", CYAN),
        SecurityPolicy::Standard => (b"STANDARD", TEXT),
        SecurityPolicy::Development => (b"DEVELOPMENT", BAD),
    };
    let facts: [(&[u8], &[u8], u32); 4] = [
        (
            b"SECURE BOOT",
            if s.secure_boot_enabled { b"ON" } else { b"OFF" },
            on(s.secure_boot_enabled),
        ),
        (
            b"TPM 2.0",
            if s.measured_boot_active { b"MEASURING" } else { b"NOT FOUND" },
            on(s.measured_boot_active),
        ),
        (
            b"ROLLBACK",
            if s.tpm_counter_ok { b"TPM COUNTER" } else { b"NO COUNTER" },
            on(s.tpm_counter_ok),
        ),
        (b"BUILD FLOOR", floor.0, floor.1),
    ];
    let half = l.col_w / 2;
    let line = metrics(Style::Mono).line + 2 * l.u;
    for (i, (name, value, color)) in facts.iter().enumerate() {
        let x = l.col_x + (i as u32 % 2) * half;
        let y = l.platform_y + (i as u32 / 2) * line;
        label(x, y, name, TEXT_3);
        label(x + label_width(b"SECURE BOOT") + 3 * l.u, y, value, *color);
    }
}

const fn on(b: bool) -> u32 {
    if b {
        CYAN
    } else {
        WARN
    }
}
