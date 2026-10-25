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

//! What the ACPI capsule tells the kernel through MkAcpiEvent(kind, value):
//! a control-method button (ACPI 6.5, 4.8.2.2), the lid (9.4), the hottest
//! thermal zone (11), a battery low or critical (10.2), and the Global Lock.

pub const POWER_BUTTON: u64 = 1;
pub const SLEEP_BUTTON: u64 = 2;
pub const LID: u64 = 3;
pub const THERMAL: u64 = 4;
pub const BATTERY_LOW: u64 = 5;
pub const BATTERY_CRITICAL: u64 = 6;
/// The ACPI capsule now serves the fixed power button itself (F-88).
pub const BUTTON_TAKEN: u64 = 7;
/// A brightness key on the output (Notify 0x86 up, 0x87 down), told to the
/// shell, which asks the display engine (F-96). Value 1 up, 0 down.
pub const BRIGHTNESS: u64 = 8;
/// The Global Lock held (1) or being given back (0), for its exit (F-107).
pub const GLOBAL_LOCK: u64 = 9;

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
    /// The charge fell below the battery's warning level.
    BatteryLow,
    /// True while critical and discharging; false once it is not.
    BatteryCritical(bool),
    /// The capsule's SCI serves the fixed power button from now on.
    ButtonTaken,
    /// True for up.
    Brightness(bool),
    GlobalLock(bool),
}

/// None for an unknown kind, or a value with bits the kind does not have.
pub fn decode(kind: u64, value: u64) -> Option<Event> {
    match (kind, value) {
        (POWER_BUTTON, 0) => Some(Event::PowerButton),
        (SLEEP_BUTTON, 0) => Some(Event::SleepButton),
        (LID, 0 | 1) => Some(Event::Lid(value == 1)),
        (THERMAL, v) if v & !THERMAL_BITS == 0 => Some(Event::Thermal(v)),
        (BATTERY_LOW, 0) => Some(Event::BatteryLow),
        (BATTERY_CRITICAL, 0 | 1) => Some(Event::BatteryCritical(value == 1)),
        (BUTTON_TAKEN, 0) => Some(Event::ButtonTaken),
        (BRIGHTNESS, 0 | 1) => Some(Event::Brightness(value == 1)),
        (GLOBAL_LOCK, 0 | 1) => Some(Event::GlobalLock(value == 1)),
        _ => None,
    }
}
