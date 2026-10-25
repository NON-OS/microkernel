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

//! ACCESSIBILITY's body read, and ACCESSIBILITY_GET's answer written: the
//! same u32 values both ways, so Settings shows what it would send.

use super::access_rule::BODY_LEN;
use super::access_rule::{Access, FILTER_TRITAN, FOLLOW_FOCUS, MAG_STEPS};

/// Settings as ACCESSIBILITY_GET answers them: filter, contrast, magnify and
/// follow, u32 each, the same values ACCESSIBILITY takes.
pub fn encode(a: &Access) -> [u8; 16] {
    let mut out = [0u8; 16];
    for (i, v) in [a.filter, a.contrast as u32, a.magnify, a.follow].iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    out
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
