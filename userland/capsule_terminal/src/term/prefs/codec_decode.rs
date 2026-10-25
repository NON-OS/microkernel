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

//! Reading saved preferences back, falling to defaults on anything unknown.

use super::{HEAD, MAGIC, VERSION};
use crate::term::dimensions::{MAX_FONT_SCALE, MIN_FONT_SCALE};
use crate::term::prefs::projects;
use crate::term::prefs::types::Prefs;
use crate::term::theme::profiles::COUNT as THEME_COUNT;

const CURSOR_COUNT: u8 = 4;
const RAILS_MASK: u8 = 0b11;

pub fn decode(b: &[u8]) -> Prefs {
    if b.len() < HEAD {
        return Prefs::default();
    }
    if b[0] != MAGIC[0] || b[1] != MAGIC[1] || b[2] != MAGIC[2] || b[3] != MAGIC[3] {
        return Prefs::default();
    }
    let version = u16::from_le_bytes([b[4], b[5]]);
    if version == 0 || version > VERSION {
        return Prefs::default();
    }
    let theme = u16::from_le_bytes([b[6], b[7]]);
    let font_scale = b[8];
    let cursor = b[9];
    let (list, count) = projects::decode_from(&b[HEAD..]);
    Prefs {
        theme: if theme < THEME_COUNT { theme } else { 0 },
        font_scale: if font_scale as u32 >= MIN_FONT_SCALE && font_scale as u32 <= MAX_FONT_SCALE {
            font_scale
        } else {
            2
        },
        cursor: if cursor < CURSOR_COUNT { cursor } else { 0 },
        rails: b[10] & RAILS_MASK,
        projects: list,
        project_count: count,
    }
}
