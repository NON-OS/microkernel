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

use alloc::string::String;

use super::index::{at, push};

static INDEX: &[u8] = include_bytes!("index/euc_kr.idx");

/* The EUC-KR decoder (windows-949): ASCII, or a lead 0x81..=0xFE and a
trail 0x41..=0xFE through the index. */
pub fn decode(b: &[u8], out: &mut String) {
    let mut i = 0;
    while let Some(&lead) = b.get(i) {
        i += 1;
        if lead < 0x80 {
            out.push(char::from(lead));
            continue;
        }
        if !(0x81..=0xFE).contains(&lead) {
            out.push('\u{FFFD}');
            continue;
        }
        let Some(&byte) = b.get(i) else { return out.push('\u{FFFD}') };
        let pointer = (0x41..=0xFE)
            .contains(&byte)
            .then(|| (usize::from(lead) - 0x81) * 190 + usize::from(byte) - 0x41);
        let c = pointer.and_then(|p| at(INDEX, p));
        push(out, c);
        /* An ASCII byte after a lead that forms nothing is read again. */
        i += usize::from(c.is_some() || byte >= 0x80);
    }
}
