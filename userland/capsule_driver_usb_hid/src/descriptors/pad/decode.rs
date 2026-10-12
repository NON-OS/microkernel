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
//! A gamepad's reports read against its layout. Buttons go down and up,
//! axes and the hat move (values.rs). Only what changed is told, and every
//! axis once on the first report that carries it.

use super::layout::{PadLayout, AXES, HAT, PAGE_BUTTON};
use super::values::{hat, read_field, scale};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PadEvent {
    Button(u16, bool),
    Axis(u16, i32),
}

pub struct PadState {
    layout: PadLayout,
    buttons: u32,
    axes: [Option<i32>; AXES],
}

impl PadState {
    pub fn new(layout: PadLayout) -> Self {
        Self { layout, buttons: 0, axes: [None; AXES] }
    }

    pub fn feed(&mut self, raw: &[u8], mut emit: impl FnMut(PadEvent)) {
        let (id, data) = match (self.layout.ids, raw.split_first()) {
            (true, Some((&id, rest))) => (id, rest),
            (false, _) => (0, raw),
            _ => return,
        };
        let mut buttons = self.buttons;
        for f in self.layout.fields.iter().filter(|f| f.report_id == id) {
            let Some(v) = read_field(data, f) else { continue };
            if f.page == PAGE_BUTTON {
                let bit = 1u32 << (f.usage - 1);
                buttons = if v != 0 { buttons | bit } else { buttons & !bit };
                continue;
            }
            let value = if f.usage == HAT { hat(v, f) } else { scale(v, f) };
            let slot = &mut self.axes[(f.usage - 0x30) as usize];
            if *slot != Some(value) {
                *slot = Some(value);
                emit(PadEvent::Axis(f.usage, value));
            }
        }
        let changed = buttons ^ self.buttons;
        for n in (0..32).filter(|n| changed >> n & 1 != 0) {
            emit(PadEvent::Button(n as u16 + 1, buttons >> n & 1 != 0));
        }
        self.buttons = buttons;
    }
}
