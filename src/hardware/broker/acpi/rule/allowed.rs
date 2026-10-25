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

//! The fixed legacy ports AML may reach, each with why: firmware's own
//! devices and conventions that no kernel path depends on. A capsule's claim
//! still takes any of them away (`ports::io` asks), and none overlaps
//! `LEGACY_PORTS`; `kernel_proofs` holds both.

#[allow(dead_code)] // the record the proofs and the letters read
pub const AML_PORTS: [(u64, u64, &str); 9] = [
    (0x60, 0x61, "8042 data: firmware polls the keyboard controller"),
    (0x61, 0x62, "NMI status: firmware times delays by its refresh bit; kernel calibrates once"),
    (0x62, 0x63, "EC data at its usual port"),
    (0x64, 0x65, "8042 status and command"),
    (0x66, 0x67, "EC command and status at its usual port"),
    (0x72, 0x74, "upper CMOS bank: firmware's settings; the clock uses 0x70-0x71"),
    (0x80, 0x81, "POST codes firmware writes"),
    (0xB2, 0xB4, "SMI command and status: firmware's own SMI interface"),
    (0x4D0, 0x4D2, "8259 edge/level: the kernel runs the I/O APIC, so the 8259 never sees it"),
];
