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

//! One DBG2 debug device information structure (Microsoft's Debug Port
//! Table 2 specification; recalled, unverified): revision u8, length u16,
//! the register count u8, the namespace string's length and offset u16,
//! the OEM data's length and offset u16, the port type and subtype u16,
//! two reserved bytes, then the offsets of the register GAS array and of
//! the u32 sizes array. Every offset is from the structure's start and
//! must land inside it.

use super::le::{u16_at, u32_at, u8_at};
use super::sdt::TableError;
use super::spcr_tail::{gas, Gas};
use alloc::vec::Vec;

const HEAD: usize = 22;
/// Registers kept for one device.
const MAX_REGISTERS: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dbg2Device {
    /// 0x8000 serial, 0x8001 1394, 0x8002 USB, 0x8003 network.
    pub port_type: u16,
    pub subtype: u16,
    /// Each register block and its size in bytes.
    pub registers: Vec<(Gas, u32)>,
    pub namespace: Vec<u8>,
}

/// The structure at `at` and its length.
pub fn device(t: &[u8], at: usize) -> Result<(Dbg2Device, usize), TableError> {
    let bad = TableError::Structure(at);
    let len = usize::from(u16_at(t, at + 1).ok_or(bad)?);
    let s = t.get(at..at.checked_add(len).ok_or(bad)?).filter(|_| len >= HEAD).ok_or(bad)?;
    let word = |o: usize| u16_at(s, o).map(usize::from).ok_or(bad);
    let count = usize::from(u8_at(s, 3).ok_or(bad)?).min(MAX_REGISTERS);
    let (regs_at, sizes_at) = (word(18)?, word(20)?);
    let registers = (0..count)
        .map(|i| Some((gas(s, regs_at + 12 * i)?, u32_at(s, sizes_at + 4 * i)?)))
        .collect::<Option<Vec<_>>>()
        .ok_or(bad)?;
    let (ns_len, ns_at) = (word(4)?, word(6)?);
    let ns = if ns_len == 0 {
        &[][..]
    } else {
        s.get(ns_at..ns_at + ns_len).filter(|_| ns_at >= HEAD).ok_or(bad)?
    };
    let namespace = ns.split(|c| *c == 0).next().unwrap_or(&[]).to_vec();
    let dev =
        Dbg2Device { port_type: word(12)? as u16, subtype: word(14)? as u16, registers, namespace };
    Ok((dev, len))
}
