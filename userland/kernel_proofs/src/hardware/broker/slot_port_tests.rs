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

//! MkAcpiSlot's ports: only a root or downstream port with a hot-plug
//! capable slot whose bridge granted native hot plug; a port's address
//! packs and unpacks whole and nothing else unpacks; and below a port is
//! exactly its bus range.

use super::acpi::slot_rule::{below, eligible};
use super::acpi::slot_wire::{pack, unpack};

#[test]
fn only_owned_hot_plug_ports() {
    for flags in 0..=0xFFFFu16 {
        let kind = (flags >> 4) & 0xF;
        for (caps, owns) in [(0x40, true), (0x40, false), (0xFFBF, true), (u32::MAX, true)] {
            let want = (kind == 4 || kind == 6) && flags & 0x100 != 0 && caps & 0x40 != 0 && owns;
            assert_eq!(eligible(flags, caps, owns), want, "{flags:#x} {caps:#x} {owns}");
        }
    }
}

#[test]
fn ports_pack_and_below_is_the_bus_range() {
    for (s, b, d, f) in [(0u16, 0u8, 0u8, 0u8), (0, 255, 31, 7), (0xFFFF, 0x12, 3, 5)] {
        assert_eq!(unpack(pack((s, b, d, f))), Some((s, b, d, f)));
    }
    for bad in [1u64, 0x800, 0x1000_0000, 1 << 48, 0x8000_0000] {
        assert_eq!(unpack(pack((0, 1, 2, 3)) | bad), None, "{bad:#x}");
    }
    for (sec, sub) in [(0u8, 9u8), (0xFF, 0xFF), (2, 4), (5, 5), (6, 3)] {
        for bus in 0..=255u8 {
            let want = sec != 0 && sec != 0xFF && sec <= bus && bus <= sub;
            assert_eq!(below(bus, sec, sub), want);
        }
    }
}
