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

//! What the ACPI capsule tells the kernel of the platform besides power,
//! through MkAcpiEvent(kind, value): a control-method button pressed
//! (ACPI 6.5, 4.8.2.2), the lid (9.4), and the hottest thermal zone (11).

pub const POWER_BUTTON: u64 = 1;
pub const SLEEP_BUTTON: u64 = 2;
pub const LID: u64 = 3;
pub const THERMAL: u64 = 4;

/// A thermal value: the zone's temperature in tenths of a kelvin, and the
/// trip points it has reached.
pub const TENTHS_K_MASK: u64 = 0xFFFF;
pub const PASSIVE: u64 = 1 << 16;
pub const HOT: u64 = 1 << 17;
pub const CRITICAL: u64 = 1 << 18;
const THERMAL_BITS: u64 = (1 << 19) - 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    PowerButton,
    SleepButton,
    /// True when open.
    Lid(bool),
    Thermal(u64),
}

/// None for an unknown kind, or a value with bits the kind does not have.
pub fn decode(kind: u64, value: u64) -> Option<Event> {
    match (kind, value) {
        (POWER_BUTTON, 0) => Some(Event::PowerButton),
        (SLEEP_BUTTON, 0) => Some(Event::SleepButton),
        (LID, 0 | 1) => Some(Event::Lid(value == 1)),
        (THERMAL, v) if v & !THERMAL_BITS == 0 => Some(Event::Thermal(v)),
        _ => None,
    }
}
