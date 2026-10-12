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

//! An output's accessibility settings, chosen in Settings and sent by the
//! shell (NCMP 0x000F): a colour filter, high contrast, and a full-screen
//! magnifier that follows the pointer or the focused window. Body, u32 each:
//! output, filter (0 none, 1 greyscale, 2 protan, 3 deutan, 4 tritan),
//! contrast (0 or 1), magnification in quarters (one of MAG_STEPS; 4 is
//! off), follow (0 the pointer, 1 the focused window).

pub const FILTER_NONE: u32 = 0;
pub const FILTER_GREYSCALE: u32 = 1;
pub const FILTER_PROTAN: u32 = 2;
pub const FILTER_DEUTAN: u32 = 3;
pub const FILTER_TRITAN: u32 = 4;

/// 1, 1.5, 2, 3, 4, 6 and 8 times.
pub const MAG_STEPS: [u32; 7] = [4, 6, 8, 12, 16, 24, 32];
pub const FOLLOW_POINTER: u32 = 0;
pub const FOLLOW_FOCUS: u32 = 1;
pub const BODY_LEN: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Access {
    pub filter: u32,
    pub contrast: bool,
    pub magnify: u32,
    pub follow: u32,
}

impl Default for Access {
    fn default() -> Self {
        Access { filter: FILTER_NONE, contrast: false, magnify: 4, follow: FOLLOW_POINTER }
    }
}

impl Access {
    pub fn magnified(&self) -> bool {
        self.magnify > 4
    }
}

/// The output and its settings, or None for a body that is not one.
pub fn parse(body: &[u8], outputs: usize) -> Option<(usize, Access)> {
    if body.len() != BODY_LEN {
        return None;
    }
    let u = |i: usize| u32::from_le_bytes([body[i], body[i + 1], body[i + 2], body[i + 3]]);
    let (out, filter, contrast, magnify, follow) = (u(0) as usize, u(4), u(8), u(12), u(16));
    let ok = out < outputs
        && filter <= FILTER_TRITAN
        && contrast <= 1
        && MAG_STEPS.contains(&magnify)
        && follow <= FOLLOW_FOCUS;
    ok.then_some((out, Access { filter, contrast: contrast == 1, magnify, follow }))
}
