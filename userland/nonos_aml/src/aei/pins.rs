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

//! The controller's _AEI: a resource template of GpioInt descriptors, each
//! naming pins of this controller that signal ACPI events. A GpioIo in it
//! is no event source and is left; a pin listed twice keeps its first
//! description.

use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use crate::serial::gpios;
use alloc::vec::Vec;

/// Pins kept of one controller; a 32-bit GPIO bank times eight.
pub const MAX_PINS: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AeiPin {
    pub pin: u16,
    /// Edge rather than level: decides between _Exx and _Lxx.
    pub edge: bool,
    /// 0 active high, 1 active low, 2 both edges.
    pub polarity: u8,
    pub shared: bool,
    pub wake: bool,
    pub debounce_centi_ms: u16,
}

/// None when the controller has no _AEI, or it fails or is no buffer.
pub fn aei_pins(aml: &mut Aml, eval: &mut Eval<'_>, controller: Id) -> Option<Vec<AeiPin>> {
    let Some(Ok(Object::Buffer(b))) = child(aml, eval, controller, *b"_AEI", Vec::new()) else {
        return None;
    };
    let mut out: Vec<AeiPin> = Vec::new();
    for g in gpios(&b.borrow()).into_iter().filter(|g| g.interrupt) {
        for pin in g.pins {
            if out.len() < MAX_PINS && !out.iter().any(|p| p.pin == pin) {
                let (edge, polarity, shared, wake) = (g.edge, g.polarity, g.shared, g.wake);
                let debounce_centi_ms = g.debounce_centi_ms;
                out.push(AeiPin { pin, edge, polarity, shared, wake, debounce_centi_ms });
            }
        }
    }
    Some(out)
}
