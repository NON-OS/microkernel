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

//! The interface `audit` keeps: its first line names the format, so a
//! script that reads the table can refuse one it was not written for. v1
//! is set down in docs/using/audit.md; a change to a column, a word a
//! reader matches, or the agent block's lines is a new version.

use alloc::vec::Vec;

pub const FORMAT: &[u8] = b"audit v1";
pub const USAGE: &[u8] = b"usage: audit [--agent <name>]";

/// The table's heading, the column a reader finds each field by.
pub const HEADING: [&[u8]; 2] = [
    b"  pid  name                  network  storage  devices*  signed by  capabilities",
    b"  ---  --------------------  -------  -------  --------  ---------  ------------",
];
/// The name column's width, its heading included.
pub const NAME_W: usize = 22;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode<'a> {
    Table,
    Agent(&'a [u8]),
}

/// The words after `audit`; Err with the line to print when they are not
/// one of the two forms.
pub fn parse<'a>(args: &[&'a [u8]]) -> Result<Mode<'a>, Vec<u8>> {
    match args {
        [] => Ok(Mode::Table),
        [b"--agent", name] if !name.is_empty() => Ok(Mode::Agent(name)),
        [b"--agent"] | [b"--agent", _] => Err(Vec::from(USAGE)),
        [first, ..] => {
            let mut e = Vec::from(&b"audit: unexpected "[..]);
            e.extend_from_slice(&printable(first));
            e.extend_from_slice(b"; ");
            e.extend_from_slice(USAGE);
            Err(e)
        }
    }
}

/// A name as the terminal may print it: anything the kernel's table holds
/// that is not plain printable ASCII becomes '?', so a capsule cannot move
/// the cursor or forge a row with its own name.
pub fn printable(name: &[u8]) -> Vec<u8> {
    name.iter().map(|b| if (0x20..0x7f).contains(b) { *b } else { b'?' }).collect()
}

/// Bytes as lowercase hex, as `receipt` prints a measurement.
pub fn hex(line: &mut Vec<u8>, bytes: &[u8]) {
    for b in bytes {
        for nib in [b >> 4, b & 0xf] {
            line.push(if nib < 10 { b'0' + nib } else { b'a' + nib - 10 });
        }
    }
}
