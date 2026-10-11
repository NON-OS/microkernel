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

//! The landed rows on Shield home (B7, v1.4.52): each fresh address a
//! refund or reclaim was paid to that holds NOX, so none is out of sight.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::order::fees::exact;
use crate::wallet::shield::landed::line;
use crate::wallet::state::State;

pub fn paint(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let mut h = 0;
    for (index, nox) in &state.meter.landed.found {
        h += fact(fb, c.x, y + h, c.w, "landed", &line(*index, &exact(*nox))) + GAP / 2;
    }
    h
}
