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
fn past_the_ceiling_the_ceiling_is_named() {
    let l = line(141_557_760, -22);
    assert_eq!(l, "gop: surface register rejected: 141557760 bytes asked; past the kernel's ceiling of 128 MiB");
}

#[test]
fn under_it_the_errno_is_named_not_the_ceiling() {
    for (rc, n) in [(-14i64, 14), (-12, 12), (-22, 22)] {
        let l = line(33_177_600, rc);
        assert_eq!(
            l,
            format!("gop: surface register rejected: 33177600 bytes asked; refused, errno {n}")
        );
        assert!(!l.contains("ceiling"), "{l}");
    }
    assert!(line(SURFACE_CEILING, -12).ends_with("errno 12"), "at the ceiling is not past it");
}

#[test]
fn it_is_said_once_however_often_setup_asks() {
    let said = AtomicBool::new(false);
    let mut lines = Vec::new();
    for _ in 0..50 {
        say_once(33_177_600, -12, &said, |l| lines.push(l.to_string()));
    }
    assert_eq!(lines, vec![line(33_177_600, -12)]);
}
