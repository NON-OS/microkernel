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

//! A GOP backing the kernel would not register is named once, with the
//! bytes it asked for and the kernel's ceiling (H-06).

use core::sync::atomic::AtomicBool;

use crate::rejected::{line, say_once, SURFACE_CEILING};

#[test]
fn the_line_names_the_bytes_and_the_ceiling() {
    let l = line(141_557_760);
    assert!(l.starts_with("gop: surface register rejected: 141557760 bytes asked;"), "{l}");
    assert!(l.contains(&format!("at most {SURFACE_CEILING} (128 MiB)")), "{l}");
}

#[test]
fn it_is_said_once_however_often_setup_asks() {
    let said = AtomicBool::new(false);
    let mut lines = Vec::new();
    for _ in 0..50 {
        say_once(141_557_760, &said, |l| lines.push(l.to_string()));
    }
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], line(141_557_760));
}
