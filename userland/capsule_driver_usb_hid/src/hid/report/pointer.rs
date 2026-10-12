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

//! A pointer report read through its layout: relative motion as a mouse
//! report, an absolute position scaled onto 0 to 0x7FFF as a tablet report.

use crate::descriptors::report::{Field, Pointer, NONE};

use super::super::mouse_event::MouseEvent;
use super::super::tablet::TabletReport;
use super::read::{flag, value};

/// The top of the range the input router scales onto the screen.
const ABS_MAX: i64 = 0x7FFF;

pub enum Motion {
    Relative(MouseEvent),
    Absolute(TabletReport),
}

/// None when the report is too short for any of its fields.
pub fn pointer_report(p: &Pointer, body: &[u8]) -> Option<Motion> {
    let (x, y) = (p.x?, p.y?);
    let mut buttons = 0u8;
    for (i, &at) in p.buttons.iter().enumerate() {
        buttons |= u8::from(flag(body, at)?) << i;
    }
    // A digitizer's tip is its first button when it names none of its own.
    let tip = flag(body, p.tip)?;
    if p.buttons[0] == NONE {
        buttons |= u8::from(tip);
    }
    let wheel = match p.wheel {
        Some(w) => value(body, w.at, w.size, w.min)?.clamp(-128, 127),
        None => 0,
    };
    let (vx, vy) = (value(body, x.at, x.size, x.min)?, value(body, y.at, y.size, y.min)?);
    if x.relative {
        let (dx, dy) = (clamp16(vx), clamp16(vy));
        return Some(Motion::Relative(MouseEvent { dx, dy, dz: wheel as i8, buttons }));
    }
    // A pen out of range or a finger lifted reports no position to go by.
    let gate = if p.range != NONE { p.range } else { p.tip };
    let at = gate == NONE || flag(body, gate)?;
    let (x, y, wheel) = (scale(vx, x), scale(vy, y), wheel as i32);
    Some(Motion::Absolute(TabletReport { buttons, x, y, wheel, at }))
}

fn clamp16(v: i64) -> i16 {
    v.clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16
}

/// `v` within the field's logical range, scaled onto 0 to ABS_MAX. The
/// layout keeps an absolute axis only when its range is not empty.
fn scale(v: i64, f: Field) -> i32 {
    let span = (f.max - f.min).max(1);
    ((v.min(f.max).max(f.min) - f.min) * ABS_MAX / span) as i32
}
