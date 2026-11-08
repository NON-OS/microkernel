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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Pill, Rect as Area, State, Tone};

use crate::browser::shell::route::Route;
use crate::browser::ui::rect::Rect;

/// The badge naming `route`: Ok on an anonymising network, plain on the
/// direct one, with or without the manual proxy.
pub fn pill(route: Route, state: State) -> Pill<'static> {
    let tone = match route {
        Route::Mixnet | Route::Anyone => Tone::Ok,
        Route::Socks | Route::Direct => Tone::Plain,
    };
    Pill::new(route.name()).tone(tone).state(state)
}

/// The box the badge for `route` takes in `slot`: its own size, held to the
/// slot, against the slot's right edge and centred down it. It is drawn and
/// pressed in this box.
pub fn body(slot: Rect, route: Route) -> Area {
    let (w, h) = pill(route, State::default()).size();
    let (sw, sh) = (slot.w.max(0) as u32, slot.h.max(0) as u32);
    let (w, h) = (w.min(sw), h.min(sh));
    let x = slot.x.max(0) as u32 + (sw - w);
    Area { x, y: slot.y.max(0) as u32 + (sh - h) / 2, w, h }
}

/// Draws the badge for `route` in `slot`, lit as `state` says.
pub fn paint(fb: &mut PaintBuffer, slot: Rect, route: Route, state: State) {
    let at = body(slot, route);
    if at.w > 0 && at.h > 0 {
        pill(route, state).draw(fb, at);
    }
}
