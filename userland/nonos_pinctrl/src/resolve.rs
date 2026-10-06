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

use crate::error::PinError;
use crate::group::Layout;

/// A pad: the community window it lives in and its index inside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pad {
    pub bar: u8,
    pub pad: u16,
}

/// The pad behind firmware pin number `gpio`, searched in table order as
/// Linux intel_gpio_to_pin does, so an overlap resolves the same way. The
/// pad index counts from the community's first pin (Linux pin_to_padno).
pub fn resolve(layout: &Layout, gpio: u32) -> Result<Pad, PinError> {
    for (bar, c) in layout.communities.iter().enumerate() {
        for g in c.groups {
            let Some(base) = g.gpio else { continue };
            let base = u32::from(base);
            if gpio < base || gpio >= base + u32::from(g.size) {
                continue;
            }
            let pin = u32::from(g.first) + (gpio - base);
            let pad = pin - u32::from(c.first);
            return Ok(Pad { bar: bar as u8, pad: pad as u16 });
        }
    }
    Err(PinError::NotMapped)
}
