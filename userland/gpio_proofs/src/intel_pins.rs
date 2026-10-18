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

//! Reaching into the Intel model by firmware pin: a pad's DW0 as the
//! registers hold it, and the line moving as the pad's trigger wants.

use super::intel_model::{place, Regs, GPI_IE, GPI_IS, PADS};
use nonos_gpio::intel::IntelGpio;
use nonos_gpio::window::Window;

pub fn dw0(g: &mut IntelGpio<Regs>, pin: u16) -> u32 {
    let p = place(pin).unwrap();
    g.communities[p.community].window.read(PADS + u32::from(p.pad) * 16)
}

pub fn set_dw0(g: &mut IntelGpio<Regs>, pin: u16, v: u32) {
    let p = place(pin).unwrap();
    g.communities[p.community].window.0.insert(PADS + u32::from(p.pad) * 16, v);
}

pub fn raise(g: &mut IntelGpio<Regs>, pin: u16) {
    let p = place(pin).unwrap();
    let w = &mut g.communities[p.community].window;
    let (is, ie) = (GPI_IS + 4 * u32::from(p.group), GPI_IE + 4 * u32::from(p.group));
    if w.read(ie) & 1 << p.bit != 0 {
        let v = w.read(is) | 1 << p.bit;
        w.0.insert(is, v);
    }
}
