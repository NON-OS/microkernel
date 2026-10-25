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

//! The `nonos-box 1` file: a header line, then `base`, `write` and `network`
//! lines in any order, and `#` comment lines. One file has one meaning, and a
//! line this reader does not know is refused, never skipped.

mod finish;
mod line;
mod words;

use alloc::vec::Vec;

pub use line::MAX_WRITES;

/// The largest policy file this reader takes.
pub const MAX_FILE: usize = 16384;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base {
    /// capsule_linux's `strict`: the run's own /tmp only.
    Strict,
    /// capsule_linux's `offline`: the run's own folders.
    Offline,
    /// `allow:`, only the folders the `write` lines name.
    Folders,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Policy {
    pub base: Base,
    /// The `write` folders, as written, in the file's order.
    pub writes: Vec<Vec<u8>>,
}

/// Why a file was refused, and on which line (1 is the first; 0 is the file
/// as a whole, for something missing).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Error {
    pub line: usize,
    pub why: &'static str,
}

pub fn parse(text: &[u8]) -> Result<Policy, Error> {
    if text.len() > MAX_FILE {
        return Err(Error { line: 0, why: "a policy file is at most 16384 bytes" });
    }
    let mut seen = line::Seen::default();
    for (i, raw) in text.split(|b| *b == b'\n').enumerate() {
        line::take(&mut seen, raw).map_err(|why| Error { line: i + 1, why })?;
    }
    finish::finish(seen)
}
