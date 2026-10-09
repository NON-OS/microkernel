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

//! Colour specifications in OSC 4, 10, 11 and 12: `rgb:r/g/b` with one to
//! four hex digits a channel, or `#rgb` to `#rrrrggggbbbb`. A `?` asks for
//! the current value, answered as xterm answers.

use crate::term::state::Term;

fn hex(s: &str) -> Option<u32> {
    if s.is_empty() || s.len() > 4 {
        return None;
    }
    let v = u32::from_str_radix(s, 16).ok()?;
    // Scale to 8 bits: one digit repeats, more keep the high byte.
    let max = (1u32 << (4 * s.len())) - 1;
    Some(v * 255 / max)
}

pub(in crate::term) fn parse_spec(spec: &[u8]) -> Option<u32> {
    let s = core::str::from_utf8(spec).ok()?;
    let (r, g, b) = if let Some(body) = s.strip_prefix("rgb:") {
        let mut it = body.split('/');
        let c = (hex(it.next()?)?, hex(it.next()?)?, hex(it.next()?)?);
        if it.next().is_some() {
            return None;
        }
        c
    } else {
        let h = s.strip_prefix('#')?;
        if h.is_empty() || h.len() % 3 != 0 || h.len() > 12 {
            return None;
        }
        let n = h.len() / 3;
        (hex(&h[..n])?, hex(&h[n..2 * n])?, hex(&h[2 * n..])?)
    };
    Some((r << 16) | (g << 8) | b)
}

impl Term {}
