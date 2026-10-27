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

//! The pointer on a framed window: on the content it is the guest's, at the
//! content's own coordinates; on the frame it lights a control, presses a
//! window button, or drags the window by its title bar.

use nonos_toolkit::decorations::{hit_test_at, DecorationHit};

use crate::linux::guest::Guest;

use super::now::shown;
use super::redraw::light;

/// The driver's left button.
const LEFT: u32 = 1;

/// A motion at (`x`, `y`) on the window: where on the content to tell the
/// guest, or None when the frame took it.
pub fn motion(guest: &mut Guest, x: i32, y: i32) -> Option<(i32, i32)> {
    let Some((a, (cw, ch, _))) = shown(&guest.scene) else { return Some((x, y)) };
    guest.scene.frame.pointer = (x, y);
    if let Some(drag) = guest.scene.frame.drag {
        super::drag::to(&mut guest.scene, drag, (x, y), a);
        return None;
    }
    if let Some(on) = a.on_content(x, y, cw, ch) {
        light(&mut guest.scene, DecorationHit::None);
        return Some(on);
    }
    let q = guest.scene.frame.quarters;
    let hit = match x >= 0 && y >= 0 {
        true => hit_test_at(a.w, a.h, false, x as u32, y as u32, q),
        false => DecorationHit::None,
    };
    light(&mut guest.scene, hit);
    None
}

/// A button press or release: true when the frame took it.
pub fn button(guest: &mut Guest, code: u32, down: bool) -> bool {
    let Some((a, (cw, ch, _))) = shown(&guest.scene) else { return false };
    let (x, y) = guest.scene.frame.pointer;
    if !down && guest.scene.frame.drag.take().is_some() {
        return true;
    }
    if !a.framed() || a.on_content(x, y, cw, ch).is_some() {
        return false;
    }
    if !down || code != LEFT || x < 0 || y < 0 {
        return true;
    }
    match hit_test_at(a.w, a.h, false, x as u32, y as u32, guest.scene.frame.quarters) {
        DecorationHit::CloseButton => super::buttons::close(guest),
        DecorationHit::MinimizeButton => super::buttons::minimize(&mut guest.scene),
        DecorationHit::MaximizeButton => super::buttons::maximize(guest),
        DecorationHit::Titlebar => super::drag::begin(&mut guest.scene, (x, y)),
        DecorationHit::None => {}
    }
    true
}
