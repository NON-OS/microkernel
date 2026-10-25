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

//! A report read back. Every check a tester's file must pass before its lines
//! become matrix rows, so a mangled, cut or hand-edited report is refused by
//! the line that is wrong, and one whose log would leak is refused outright.

use crate::parse_line::judge;
use crate::MAX_LINE;

/// What is wrong, and on which line, counted from 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fault {
    NoHeader,
    TooLong(usize),
    Field(usize),
    /// A log line that names a private word or carries an unmasked value.
    Leak(usize),
    NoEnd,
    Count(usize),
    AfterEnd(usize),
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Summary {
    pub pci: u32,
    pub acpi: u32,
    pub log: u32,
    pub dropped: u32,
    pub unconfined: bool,
    pub cut: bool,
}

pub fn check(report: &[u8]) -> Result<Summary, Fault> {
    let body = report.strip_suffix(b"\n").ok_or(Fault::NoEnd)?;
    let mut s = Summary::default();
    let mut ended = false;
    for (i, line) in body.split(|b| *b == b'\n').enumerate() {
        if ended {
            return Err(Fault::AfterEnd(i + 1));
        }
        if line.len() > MAX_LINE {
            return Err(Fault::TooLong(i + 1));
        }
        ended = judge(i, line, &mut s)?;
    }
    if ended {
        Ok(s)
    } else {
        Err(Fault::NoEnd)
    }
}
