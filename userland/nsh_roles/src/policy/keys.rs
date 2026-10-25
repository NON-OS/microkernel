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

//! The role authority keys and the approver keys a table lists. Each list
//! is in strictly ascending key-id order, so a list never names a key twice
//! and one set of keys has one encoding. No key is in both lists: a key
//! that issues grants never also approves them.

use crate::hybrid::Key;
use crate::policy::within;
use crate::PolicyError;
use alloc::vec::Vec;
use nsh_wire::keys::HybridPublic;
use nsh_wire::Reader;

pub(crate) fn keys<'a>(
    r: &mut Reader<'a>,
    min: usize,
    max: usize,
) -> Result<Vec<Key<'a>>, PolicyError> {
    let n = within(u64::from(r.u8()?), min as u64, max as u64)? as usize;
    let mut out: Vec<Key<'a>> = Vec::with_capacity(n);
    for _ in 0..n {
        let key = Key::new(HybridPublic::read(r)?);
        if out.last().is_some_and(|before| key.id <= before.id) {
            return Err(PolicyError::Order);
        }
        out.push(key);
    }
    Ok(out)
}

pub(crate) fn disjoint(a: &[Key<'_>], b: &[Key<'_>]) -> Result<(), PolicyError> {
    match a.iter().any(|x| b.iter().any(|y| x.id == y.id)) {
        true => Err(PolicyError::Overlap),
        false => Ok(()),
    }
}
