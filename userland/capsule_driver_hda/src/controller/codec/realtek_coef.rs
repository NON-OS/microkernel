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

//! Codec coefficients, read and written through the vendor widget's
//! SET_COEF_INDEX and GET/SET_PROC_COEF.

use crate::constants::{VERB_GET_PROC_COEF, VERB_SET_COEF_INDEX, VERB_SET_PROC_COEF};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

pub(super) const COEF_NID: u8 = 0x20;

/// One read-modify-write of a coefficient: clear `mask`, set `set`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CoefUpdate {
    pub nid: u8,
    pub idx: u16,
    pub mask: u16,
    pub set: u16,
}

pub(super) const fn upd(idx: u16, mask: u16, set: u16) -> CoefUpdate {
    CoefUpdate { nid: COEF_NID, idx, mask, set }
}

pub fn read_coef(link: &mut Link, cad: u8, nid: u8, idx: u16) -> HdaResult<u16> {
    link.send(compose_verb_long(cad, nid, VERB_SET_COEF_INDEX as u16, idx))?;
    Ok(link.send(compose_verb(cad, nid, VERB_GET_PROC_COEF, 0))? as u16)
}

pub fn write_coef(link: &mut Link, cad: u8, nid: u8, idx: u16, val: u16) -> HdaResult<()> {
    link.send(compose_verb_long(cad, nid, VERB_SET_COEF_INDEX as u16, idx))?;
    link.send(compose_verb_long(cad, nid, VERB_SET_PROC_COEF as u16, val))?;
    Ok(())
}

pub fn update(link: &mut Link, cad: u8, u: CoefUpdate) -> HdaResult<()> {
    let v = read_coef(link, cad, u.nid, u.idx)?;
    write_coef(link, cad, u.nid, u.idx, (v & !u.mask) | u.set)
}
