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

//! The touchpad's report: the first input report of a Touch Pad application
//! collection that carries a Tip Switch, with its fields, and the
//! configuration switches the Feature reports declare. A precision touchpad
//! declares its mouse collection first; a touch screen, a pen or a keyboard
//! is no touch pad, and none of their fields is taken.

use super::assign::{assign, TIP_SWITCH};
use super::feature::feature;
use super::walk::walk;
use crate::hid::report_desc::layout::TouchLayout;

/// Usage Page Digitizer, Usage Touch Pad.
const TOUCH_PAD: u32 = 0x000D_0005;

/// The touch layout of `desc`; empty when the walk refuses the descriptor
/// or it declares no touch pad.
pub fn parse(desc: &[u8]) -> TouchLayout {
    let mut id = None;
    let found = walk(desc, &mut |f| {
        if id.is_none() && !f.feature && f.app == TOUCH_PAD && f.usage == TIP_SWITCH {
            id = Some(f.id);
        }
    });
    // A report too long to read whole is never decoded (long.rs).
    let (Ok(long), Some(id)) = (found, id) else { return TouchLayout::default() };
    if long.has(id) {
        return TouchLayout::default();
    }
    let mut l = TouchLayout { report_id: id, ..TouchLayout::default() };
    let laid = walk(desc, &mut |f| {
        if f.feature {
            feature(&mut l, f);
        } else if f.app == TOUCH_PAD && f.id == id {
            assign(&mut l, f);
        }
    });
    laid.map_or_else(|_| TouchLayout::default(), |_| l)
}
