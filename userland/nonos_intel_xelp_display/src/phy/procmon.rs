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
//! The process monitor reference values a PHY is initialised with, picked
//! by the voltage and process PORT_COMP_DW3 reports (Vol 12 p. 392; the
//! DW3 fields, Vol 2c p. 899).

/// The bytes 7:0 and 23:16 of PORT_COMP_DW1, then DW9 and DW10 whole.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Procmon {
    pub dw1_low: u32,
    pub dw1_high: u32,
    pub dw9: u32,
    pub dw10: u32,
}

/// The values for the voltage (DW3 25:24: 00b 0.85 V, 01b 0.95 V, 10b
/// 1.05 V) and process (28:26: 000b dot-0, 001b dot-1) that `dw3` holds.
/// None for any pair Vol 12's table has no column for: 0.85 V dot-1 is
/// marked N/A there, and dot-4 and the reserved codes are not listed.
pub fn for_dw3(dw3: u32) -> Option<Procmon> {
    let voltage = (dw3 >> 24) & 3;
    let process = (dw3 >> 26) & 7;
    let (dw1_high, dw9, dw10) = match (voltage, process) {
        (0, 0) => (0x00, 0x62AB_67BB, 0x5191_4F96),
        (1, 0) => (0x00, 0x86E1_72C7, 0x77CA_5EAB),
        (1, 1) => (0x00, 0x93F8_7FE1, 0x8AE8_71C5),
        (2, 0) => (0x00, 0x98FA_82DD, 0x89E4_6DC1),
        (2, 1) => (0x44, 0x9A00_AB25, 0x8AE3_8FF1),
        _ => return None,
    };
    Some(Procmon { dw1_low: 0x00, dw1_high, dw9, dw10 })
}
