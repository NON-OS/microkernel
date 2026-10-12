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

//! WDAT as firmware lays it out (recalled from Microsoft's description,
//! unverified), and a bus that records what an action did.

use super::wdat_decode::Reg;
use super::wdat_run::WdatBus;
use std::collections::BTreeMap;

pub const IO: u8 = 1;

/// One instruction: action, flags (kind | 0x80 to preserve), register.
pub fn entry(action: u8, flags: u8, space: u8, address: u64, value: u32, mask: u32) -> Vec<u8> {
    let gas = [vec![space, 32, 0, 3], address.to_le_bytes().to_vec()].concat();
    let tail = [value.to_le_bytes(), mask.to_le_bytes()].concat();
    [vec![action, flags, 0, 0], gas, tail].concat()
}

/// The table: period 1000 ms, counts 1 to 600, enabled, these entries.
pub fn table(entries: &[Vec<u8>], count: u32) -> Vec<u8> {
    let mut t = b"WDAT".to_vec();
    t.extend_from_slice(&[0; 32]);
    t.extend_from_slice(&32u32.to_le_bytes());
    t.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0]);
    for v in [1000u32, 600, 1] {
        t.extend_from_slice(&v.to_le_bytes());
    }
    t.extend_from_slice(&[0x81, 0, 0, 0]);
    t.extend_from_slice(&count.to_le_bytes());
    t.extend_from_slice(&entries.concat());
    t
}

/// Registers by address; every read and write recorded in order.
#[derive(Default)]
pub struct Bus {
    pub regs: BTreeMap<u64, u64>,
    pub log: Vec<(char, u64, u64)>,
}

impl WdatBus for Bus {
    fn read(&mut self, r: &Reg) -> Option<u64> {
        let v = *self.regs.get(&r.address)?;
        self.log.push(('r', r.address, v));
        Some(v)
    }
    fn write(&mut self, r: &Reg, v: u64) -> bool {
        self.log.push(('w', r.address, v));
        self.regs.insert(r.address, v);
        true
    }
}
