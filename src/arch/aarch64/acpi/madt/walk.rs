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

//! The MADT's structures one by one after its fixed 44 bytes: each one's
//! length at least two and inside the table, and at least what its type
//! needs; any that fails stops the walk with its offset. Types other than
//! the four GIC ones read here are passed over by their length.

use super::super::sdt::{table, TableError, HEADER};
use super::gicc::{Gicc, MIN as GICC_MIN};
use super::parts::{gicd, gicr, its, Gicd, Gicr, Its, GICD_LEN, GICR_LEN, ITS_LEN};
use alloc::vec::Vec;

const STRUCTURES: usize = HEADER + 8;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gic {
    pub gicc: Vec<Gicc>,
    pub gicd: Vec<Gicd>,
    pub gicr: Vec<Gicr>,
    pub its: Vec<Its>,
}

pub fn madt_gic(t: &[u8]) -> Result<Gic, TableError> {
    let t = table(t, b"APIC")?;
    if t.len() < STRUCTURES {
        return Err(TableError::Short);
    }
    let (mut gic, mut at) = (Gic::default(), STRUCTURES);
    while at < t.len() {
        let bad = TableError::Structure(at);
        let len = *t.get(at + 1).ok_or(bad)? as usize;
        let s = t.get(at..at + len).filter(|_| len >= 2).ok_or(bad)?;
        let need = match s[0] {
            0x0B => GICC_MIN,
            0x0C => GICD_LEN,
            0x0E => GICR_LEN,
            0x0F => ITS_LEN,
            _ => 2,
        };
        if len < need {
            return Err(bad);
        }
        match s[0] {
            0x0B => gic.gicc.push(Gicc::read(s).ok_or(bad)?),
            0x0C => gic.gicd.push(gicd(s).ok_or(bad)?),
            0x0E => gic.gicr.push(gicr(s).ok_or(bad)?),
            0x0F => gic.its.push(its(s).ok_or(bad)?),
            _ => {}
        }
        at += len;
    }
    Ok(gic)
}
