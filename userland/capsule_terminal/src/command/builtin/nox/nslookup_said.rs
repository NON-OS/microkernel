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

//! The line a resolved host prints: the name and its address.

use alloc::vec::Vec;

use super::HDR_LEN;
use crate::term::util::format_u64;

/// `host -> a.b.c.d`, the address read from the reply in `rx`.
pub(super) fn answer_line(host: &[u8], rx: &[u8]) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(host);
    line.extend_from_slice(b" -> ");
    for (i, &b) in rx[HDR_LEN..HDR_LEN + 4].iter().enumerate() {
        if i > 0 {
            line.push(b'.');
        }
        let mut buf = [0u8; 24];
        let k = format_u64(b as u64, &mut buf);
        line.extend_from_slice(&buf[..k]);
    }
    line
}
