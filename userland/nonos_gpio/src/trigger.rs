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

//! How a GpioInt pin interrupts, as its descriptor says (ACPI 6.5,
//! 6.4.3.8.1): on an edge or while a level holds, and at which polarity
//! (active high, active low, or, for edges, both).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    High,
    Low,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trigger {
    pub edge: bool,
    pub polarity: Polarity,
}

impl Trigger {
    /// From the descriptor's interrupt flags: bit 0 edge, bits 2:1 the
    /// polarity (0 high, 1 low, 2 both). None for a level trigger on both
    /// polarities, or polarity 3, which no pin can honour.
    pub fn from_flags(flags: u16) -> Option<Trigger> {
        let edge = flags & 1 != 0;
        let polarity = match (flags >> 1) & 3 {
            0 => Polarity::High,
            1 => Polarity::Low,
            2 if edge => Polarity::Both,
            _ => return None,
        };
        Some(Trigger { edge, polarity })
    }
}
