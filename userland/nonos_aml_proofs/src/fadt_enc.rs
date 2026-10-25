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

//! FADTs for the event proofs: q35's layout, with the legacy fields, and the
//! same blocks given again through the X_ fields where a test asks.

/// PM1a event block 0x600 (4 bytes), PM1a control 0x604 (2), GPE0 0x620
/// (16 bytes: 64 events), SCI 9, SMI_CMD 0xB2 with ACPI_ENABLE 0x02.
pub const PM1_STS: u16 = 0x600;
pub const PM1_EN: u16 = 0x602;
pub const PM1_CNT: u16 = 0x604;
pub const GPE0: u16 = 0x620;

fn put(t: &mut [u8], at: usize, bytes: &[u8]) {
    t[at..at + bytes.len()].copy_from_slice(bytes);
}

pub fn q35(len: usize) -> Vec<u8> {
    let mut t = vec![0u8; len];
    put(&mut t, 0, b"FACP");
    put(&mut t, 4, &(len as u32).to_le_bytes());
    put(&mut t, 46, &9u16.to_le_bytes());
    put(&mut t, 48, &0xB2u32.to_le_bytes());
    t[52] = 0x02;
    t[53] = 0x03;
    put(&mut t, 56, &u32::from(PM1_STS).to_le_bytes());
    put(&mut t, 64, &u32::from(PM1_CNT).to_le_bytes());
    put(&mut t, 80, &u32::from(GPE0).to_le_bytes());
    t[88] = 4;
    t[89] = 2;
    t[92] = 16;
    t
}

/// A Generic Address Structure at `at`.
pub fn x(t: &mut [u8], at: usize, space: u8, width: u8, address: u64) {
    put(t, at, &[space, width, 0, 0]);
    put(t, at + 4, &address.to_le_bytes());
}

/// A second GPE block at `port` of `len` bytes, numbered from `base`.
pub fn gpe1(t: &mut [u8], port: u32, len: u8, base: u8) {
    put(t, 84, &port.to_le_bytes());
    t[93] = len;
    t[94] = base;
}
