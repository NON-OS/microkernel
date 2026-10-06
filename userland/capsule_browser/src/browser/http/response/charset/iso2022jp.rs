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

use super::index::{at, push, JIS0208};
use super::iso2022jp_state::{escape, St};

/* The ISO-2022-JP decoder: escape sequences switch between ASCII, JIS
X 0201 Roman, half-width katakana and JIS X 0208 byte pairs. Two
escapes with nothing decoded between them are an error, as the
standard requires. */
pub fn decode(b: &[u8], out: &mut String) {
    let mut s = St::start();
    let mut i = 0;
    loop {
        let byte = b.get(i).copied();
        if s.state == St::ESC_START || s.state == St::ESC {
            i = escape(&mut s, byte, i, out);
            continue;
        }
        /* A trail byte that never came, or an escape in its place, is an error. */
        let broken_pair = s.state == St::TRAIL && byte.is_none_or(|c| c == 0x1B);
        if broken_pair {
            out.push('\u{FFFD}');
        }
        let Some(c) = byte else { return };
        i += 1;
        if c == 0x1B {
            s.state = St::ESC_START;
            continue;
        }
        if s.state == St::TRAIL {
            s.state = St::LEAD;
            let c = (0x21..=0x7E)
                .contains(&c)
                .then(|| usize::from(s.lead - 0x21) * 94 + usize::from(c) - 0x21);
            push(out, c.and_then(|p| at(JIS0208, p)));
            continue;
        }
        s.output = false;
        let cp = match (s.state, c) {
            (St::ASCII, 0x00..=0x7F) if c != 0x0E && c != 0x0F => Some(u32::from(c)),
            (St::ROMAN, 0x5C) => Some(0xA5),
            (St::ROMAN, 0x7E) => Some(0x203E),
            (St::ROMAN, 0x00..=0x7F) if c != 0x0E && c != 0x0F => Some(u32::from(c)),
            (St::KATAKANA, 0x21..=0x5F) => Some(0xFF61 - 0x21 + u32::from(c)),
            (St::LEAD, 0x21..=0x7E) => {
                (s.lead, s.state) = (c, St::TRAIL);
                continue;
            }
            _ => None,
        };
        push(out, cp);
    }
}
