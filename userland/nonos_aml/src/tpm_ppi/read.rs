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

//! Reading the PPI: the first present TPM device, function 0's bitmask
//! (bit 0 says the UUID is supported at all), function 1's version and,
//! when function 8 is there, its answer for each operation up to `MAX_OP`.

use super::dsm::{call, QUERY, USER_CONFIRMATION, VERSION};
use super::TPM_IDS;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{DeviceInfo, Eval, PRESENT};
use alloc::vec::Vec;

/// TCG's operations sit below this; vendor ones (128 up) are not asked.
pub const MAX_OP: u8 = 63;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ppi {
    pub dev: Id,
    pub functions: u64,
    pub version: Option<Vec<u8>>,
    /// Each operation function 8 did not answer 0 for, with its answer.
    pub ops: Vec<(u8, u8)>,
}

pub fn read_ppi(aml: &mut Aml, eval: &mut Eval<'_>, devices: &[DeviceInfo]) -> Option<Ppi> {
    let tpm = |d: &&DeviceInfo| d.hid.iter().chain(&d.cids).any(|h| TPM_IDS.contains(&h.bytes()));
    let dev = devices.iter().filter(|d| d.sta & PRESENT != 0).find(tpm)?.id;
    let functions = match call(aml, eval, dev, QUERY, None)? {
        Object::Buffer(b) => b.borrow().iter().take(8).rev().fold(0, |m, b| m << 8 | u64::from(*b)),
        Object::Integer(n) => n,
        _ => return None,
    };
    if functions & 1 == 0 {
        return None;
    }
    let version = (functions & 1 << VERSION != 0)
        .then(|| match call(aml, eval, dev, VERSION, None)? {
            Object::String(s) => Some(s.borrow().iter().copied().take_while(|c| *c != 0).collect()),
            _ => None,
        })
        .flatten();
    let mut ops = Vec::new();
    if functions & 1 << USER_CONFIRMATION != 0 {
        for op in 0..=MAX_OP {
            if let Some(Object::Integer(n @ 1..=4)) =
                call(aml, eval, dev, USER_CONFIRMATION, Some(u64::from(op)))
            {
                ops.push((op, n as u8));
            }
        }
    }
    Some(Ppi { dev, functions, version, ops })
}
