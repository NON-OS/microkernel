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

use super::device_sites::device_sites;
use super::id_sites::id_sites;

/// The trimmed lines of `src` that cast to a narrower integer, comments aside.
fn narrowing_casts(src: &str) -> Vec<&str> {
    const CASTS: [&str; 6] = [" as u32", " as u16", " as u8", " as i32", " as i16", " as i8"];
    src.lines()
        .map(str::trim)
        .filter(|l| !(l.starts_with("//") || l.starts_with('*') || l.starts_with("/*")))
        .filter(|l| {
            CASTS.iter().any(|c| {
                l.match_indices(c).any(|(i, _)| {
                    !l[i + c.len()..].starts_with(|ch: char| ch.is_ascii_alphanumeric())
                })
            })
        })
        .collect()
}

#[test]
fn no_id_call_site_truncates_a_register() {
    for (name, src, kept) in id_sites() {
        assert_eq!(narrowing_casts(src), kept, "{name} narrows a register with a cast");
    }
}

#[test]
fn no_device_call_site_truncates_a_register() {
    for (name, src, kept) in device_sites() {
        assert_eq!(narrowing_casts(src), kept, "{name} narrows a register with a cast");
    }
}

#[test]
fn the_scan_sees_the_casts_it_was_written_for() {
    assert_eq!(narrowing_casts("    let target = pid as u32;"), ["let target = pid as u32;"]);
    assert_eq!(narrowing_casts("lookup_port(ep as u32)"), ["lookup_port(ep as u32)"]);
    assert!(narrowing_casts("// as u32 in a comment\nlet n = len as usize;").is_empty());
    assert!(narrowing_casts("let w = x as u32x4;").is_empty());
}
