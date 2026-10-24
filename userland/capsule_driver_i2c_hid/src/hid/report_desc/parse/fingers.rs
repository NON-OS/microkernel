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

//! Every finger of a touchscreen's or a parallel-mode touchpad's report:
//! the first input report of a Touch Screen (04h) or Touch Pad (05h)
//! application collection on the Digitizers page that carries a Tip
//! Switch (HID Usage Tables, as recalled, unverified), each of its fields
//! given to the slots in the order the descriptor lays them out
//! (multitouch/slots.rs), with its Contact Count and clickpad button.

use super::assign::TIP_SWITCH;
use super::field_at::FieldAt;
use super::walk::walk;
use crate::hid::report_desc::multitouch::Slots;
use crate::hid::report_desc::Field;

const TOUCH_SCREEN: u32 = 0x000D_0004;
const TOUCH_PAD: u32 = 0x000D_0005;
const CONTACT_COUNT: u32 = 0x000D_0054;
const BUTTON_PAGE: u32 = 0x0009;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Surface {
    #[default]
    Pad,
    Screen,
}

#[derive(Clone, Copy, Default)]
pub struct Fingers {
    pub surface: Surface,
    pub rid: u8,
    pub slots: Slots,
    pub count: Field,
    pub button: Field,
}

fn digitizer(f: &FieldAt) -> bool {
    !f.feature && (f.app == TOUCH_SCREEN || f.app == TOUCH_PAD)
}

/// The fingers of `desc`; none (no slot) when the walk refuses it, it
/// declares no touch report, or that report is too long to read whole.
pub fn parse_fingers(desc: &[u8]) -> Fingers {
    let mut first = None;
    let found = walk(desc, &mut |f| {
        if first.is_none() && digitizer(f) && f.usage == TIP_SWITCH {
            first = Some((f.id, f.app));
        }
    });
    let (Ok(long), Some((rid, app))) = (found, first) else { return Fingers::default() };
    if long.has(rid) {
        return Fingers::default();
    }
    let surface = if app == TOUCH_SCREEN { Surface::Screen } else { Surface::Pad };
    let mut g = Fingers { surface, rid, ..Fingers::default() };
    let laid = walk(desc, &mut |f| match f.usage {
        _ if !digitizer(f) || f.id != rid || f.app != app => {}
        CONTACT_COUNT if !g.count.present() => g.count = f.field(),
        u if u >> 16 == BUTTON_PAGE && !g.button.present() => g.button = f.field(),
        u => g.slots.record((u >> 16) as u16, u as u16, f.field()),
    });
    laid.map_or_else(|_| Fingers::default(), |_| g)
}
