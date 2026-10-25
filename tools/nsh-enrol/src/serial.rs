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

//! The document a development image prints after its boot, read back from
//! the serial log: the last one, between `[ATTEST-DOC] begin N` and `end`,
//! whole and exactly `N` bytes, never more than a document can be.

use nsh_attest::consts::MAX_DOC;

use crate::{hex, Error};

const TAG: &str = "[ATTEST-DOC] ";

pub fn doc_from_serial(log: &str) -> Result<Vec<u8>, Error> {
    let lines: Vec<&str> =
        log.lines().filter_map(|l| l.split_once(TAG).map(|(_, r)| r.trim())).collect();
    let begin = lines
        .iter()
        .rposition(|l| l.starts_with("begin "))
        .ok_or(Error::Value("serial: no document"))?;
    let len: usize = lines[begin][6..].parse().map_err(|_| Error::Value("serial: length"))?;
    if len > MAX_DOC {
        return Err(Error::Value("serial: longer than a document"));
    }
    let mut out = Vec::with_capacity(len);
    for l in &lines[begin + 1..] {
        if *l == "end" {
            return match out.len() == len {
                true => Ok(out),
                false => Err(Error::Value("serial: document cut")),
            };
        }
        if l.len() % 2 != 0 || out.len() + l.len() / 2 > len {
            return Err(Error::Value("serial: hex"));
        }
        for i in (0..l.len()).step_by(2) {
            out.push(hex::array::<1>(&l[i..i + 2], "serial: hex")?[0]);
        }
    }
    Err(Error::Value("serial: no end line"))
}
