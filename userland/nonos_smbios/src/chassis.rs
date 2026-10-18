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

//! The chassis type (DSP0134 7.4.1): the low seven bits of the System
//! Enclosure structure's type byte; the top bit says a lock is present.

pub fn chassis_name(t: u8) -> &'static str {
    const NAMES: [&str; 36] = [
        "Other",
        "Unknown",
        "Desktop",
        "Low Profile Desktop",
        "Pizza Box",
        "Mini Tower",
        "Tower",
        "Portable",
        "Laptop",
        "Notebook",
        "Hand Held",
        "Docking Station",
        "All in One",
        "Sub Notebook",
        "Space-saving",
        "Lunch Box",
        "Main Server Chassis",
        "Expansion Chassis",
        "SubChassis",
        "Bus Expansion Chassis",
        "Peripheral Chassis",
        "RAID Chassis",
        "Rack Mount Chassis",
        "Sealed-case PC",
        "Multi-system chassis",
        "Compact PCI",
        "Advanced TCA",
        "Blade",
        "Blade Enclosure",
        "Tablet",
        "Convertible",
        "Detachable",
        "IoT Gateway",
        "Embedded PC",
        "Mini PC",
        "Stick PC",
    ];
    usize::from(t & 0x7F).checked_sub(1).and_then(|i| NAMES.get(i)).copied().unwrap_or("Unknown")
}
