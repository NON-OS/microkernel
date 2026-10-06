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

/* ISO-2022-JP decoder state: the current state, the state an escape
returns to, the byte kept between steps, and whether the last escape
had nothing decoded after it yet. */
pub struct St {
    pub state: u8,
    pub out_state: u8,
    pub lead: u8,
    pub output: bool,
}

impl St {
    pub const ASCII: u8 = 0;
    pub const ROMAN: u8 = 1;
    pub const KATAKANA: u8 = 2;
    pub const LEAD: u8 = 3;
    pub const TRAIL: u8 = 4;
    pub const ESC_START: u8 = 5;
    pub const ESC: u8 = 6;

    pub fn start() -> St {
        St { state: St::ASCII, out_state: St::ASCII, lead: 0, output: false }
    }
}

/* One step of the escape start and escape states for `byte` (None at
the end) at input index `i`; returns the index to read next, which
steps back where the standard restores bytes to the input. */
pub fn escape(s: &mut St, byte: Option<u8>, i: usize, out: &mut String) -> usize {
    if s.state == St::ESC_START {
        if let Some(c @ (0x24 | 0x28)) = byte {
            (s.lead, s.state) = (c, St::ESC);
            return i + 1;
        }
        (s.output, s.state) = (false, s.out_state);
        out.push('\u{FFFD}');
        return i;
    }
    let lead = core::mem::take(&mut s.lead);
    let next = match (lead, byte) {
        (0x28, Some(0x42)) => Some(St::ASCII),
        (0x28, Some(0x4A)) => Some(St::ROMAN),
        (0x28, Some(0x49)) => Some(St::KATAKANA),
        (0x24, Some(0x40 | 0x42)) => Some(St::LEAD),
        _ => None,
    };
    if let Some(n) = next {
        (s.state, s.out_state) = (n, n);
        if core::mem::replace(&mut s.output, true) {
            out.push('\u{FFFD}');
        }
        return i + 1;
    }
    (s.output, s.state) = (false, s.out_state);
    out.push('\u{FFFD}');
    i - 1
}
