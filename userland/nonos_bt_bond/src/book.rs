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

//! The bonded peers this host knows. A peer is found by its identity, or,
//! when it advertises from a resolvable private address, by the bond whose
//! IRK resolves it (Vol 6, Part B, 1.3.2.3). Fifteen bonds: one owner's
//! sixteen keyring records, less this host's own IRK.

use crate::error::BondError;
use crate::record::BondRecord;
use alloc::vec::Vec;
use nonos_bt_smp::privacy::resolves;
use nonos_bt_wire::addr::{AddrType, BdAddr, RandomKind};

pub const MAX_BONDS: usize = 15;

#[derive(Default)]
pub struct Book {
    bonds: Vec<BondRecord>,
}

impl Book {
    /// The bond for a peer seen at `addr`, if any.
    pub fn find(&self, kind: AddrType, addr: &BdAddr) -> Option<&BondRecord> {
        let same = |t: u8| move |b: &&BondRecord| b.identity_type == t && b.identity == addr.0;
        match (kind, addr.random_kind()) {
            (AddrType::Public, _) => self.bonds.iter().find(same(0)),
            (AddrType::Random, Ok(RandomKind::Static)) => self.bonds.iter().find(same(1)),
            (AddrType::Random, Ok(RandomKind::Resolvable)) => {
                self.bonds.iter().find(|b| b.irk.as_ref().is_some_and(|k| resolves(k, addr)))
            }
            _ => None,
        }
    }

    /// Keep `record`, in place of an older bond with the same identity.
    pub fn insert(&mut self, record: BondRecord) -> Result<(), BondError> {
        record.check()?;
        let at = self.position(record.identity_type, &record.identity);
        match at {
            Some(i) => self.bonds[i] = record,
            None if self.bonds.len() >= MAX_BONDS => return Err(BondError::Full),
            None => self.bonds.push(record),
        }
        Ok(())
    }

    /// Forget the bond with this identity; whether there was one.
    pub fn remove(&mut self, identity_type: u8, identity: &[u8; 6]) -> bool {
        let at = self.position(identity_type, identity);
        at.map(|i| self.bonds.remove(i)).is_some()
    }

    pub fn bonds(&self) -> &[BondRecord] {
        &self.bonds
    }

    fn position(&self, t: u8, id: &[u8; 6]) -> Option<usize> {
        self.bonds.iter().position(|b| b.identity_type == t && b.identity == *id)
    }
}
