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

//! The headers of a 200 to a GET: one Content-Length, a number, at most
//! 1 MiB. A Transfer-Encoding, a second length or a line that is not a
//! header is refused.

use nonos_proof_abi::MAX_BYTES;

/// Not an answer fetch takes.
#[derive(PartialEq, Eq, Debug)]
pub struct Refused;

pub fn length<'a>(lines: impl Iterator<Item = &'a str>) -> Result<usize, Refused> {
    let mut len = None;
    for line in lines {
        let (name, value) = line.split_once(':').ok_or(Refused)?;
        match name.trim().to_ascii_lowercase().as_str() {
            "transfer-encoding" => return Err(Refused),
            "content-length" if len.is_none() => len = Some(value.trim().parse::<usize>().ok()),
            "content-length" => return Err(Refused),
            _ => {}
        }
    }
    len.flatten().filter(|&n| n <= MAX_BYTES).ok_or(Refused)
}
