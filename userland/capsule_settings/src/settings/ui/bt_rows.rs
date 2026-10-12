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

//! One bonded device: its identity address, how it was paired, and, on the
//! right, whether the link runs on it and its battery.

use nonos_app_skeleton::PaintBuffer;

use crate::bluetooth::panel::{addr_text, level_of, made, unreached, Reach};
use crate::settings::state::State;

use super::bytes::as_str;
use super::control_geom::right_edge;
use super::metrics::BODY_PX;
use super::row_focus;
use super::row_label;
use super::text;
use super::theme::{OK, SUBLABEL_FG};
use super::valbuf::ValBuf;

pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    index: usize,
    card_x: u32,
    card_w: u32,
    screen_y: i32,
    row_h: u32,
) {
    let p = &state.bt;
    if p.bond_count == 0 {
        let why = match p.reach {
            Reach::Answered => "None paired. P lets one pair",
            Reach::Unasked => "--",
            reach => unreached(reach),
        };
        row_label::paint(fb, card_x, screen_y, row_h, why, None);
        return;
    }
    let i = index.min(p.bond_count - 1);
    if p.cursor == i {
        row_focus::paint(fb, card_x, card_w, screen_y, row_h);
    }
    let bond = p.bonds[i];
    let addr = addr_text(&bond.addr);
    row_label::paint(fb, card_x, screen_y, row_h, as_str(&addr), Some(made(&bond)));
    let mut v = ValBuf::new();
    if bond.linked {
        v.push_str("Connected");
    }
    if let Some(percent) = level_of(p, i) {
        v.push_str(if bond.linked { ", battery " } else { "Battery " });
        v.push_u32(u32::from(percent));
        v.push_str("%");
    }
    let colour = if bond.linked { OK } else { SUBLABEL_FG };
    let top = text::centred_top(0, row_h, BODY_PX) + screen_y;
    text::right(fb, right_edge(card_x, card_w), top, v.as_str(), colour, BODY_PX);
}
