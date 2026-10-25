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

//! The id list, one id per line: the id, its class, and where the id is
//! documented, tab-separated. The list is data so that an id can be added
//! or its source corrected without touching the interpreter.

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    I2c,
    Sd,
    Gpio,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Known<'a> {
    pub id: &'a [u8],
    pub class: Class,
    pub source: &'a [u8],
}

fn line(l: &[u8]) -> Option<Known<'_>> {
    let mut f = l.splitn(3, |c| *c == b'\t');
    let (id, class, source) = (f.next()?, f.next()?, f.next()?);
    // Eight characters is the longest a _HID string is (ACPI 6.5, 6.1.5).
    let id_ok = (1..=8).contains(&id.len()) && id.iter().all(|c| c.is_ascii_alphanumeric());
    let class = match class {
        b"i2c" => Class::I2c,
        b"sd" => Class::Sd,
        b"gpio" => Class::Gpio,
        _ => return None,
    };
    (id_ok && !source.is_empty()).then_some(Known { id, class, source })
}

/// The entries that hold, and how many lines did not: a line that is
/// malformed is dropped and counted rather than guessed at, so the caller
/// can say so. Blank lines and lines starting with '#' are neither.
pub fn known(text: &[u8]) -> (Vec<Known<'_>>, usize) {
    let (mut out, mut dropped) = (Vec::new(), 0);
    for l in text.split(|c| *c == b'\n') {
        let l = l.strip_suffix(b"\r").unwrap_or(l);
        if l.is_empty() || l[0] == b'#' {
            continue;
        }
        match line(l) {
            Some(k) => out.push(k),
            None => dropped += 1,
        }
    }
    (out, dropped)
}
