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

//! The book, where its records are kept, and the id each bond is kept
//! under, by identity.

use crate::book::Book;
use crate::record::BondRecord;
use nonos_bt_wire::addr::{AddrType, BdAddr};

pub struct Store<K> {
    pub(super) keys: K,
    pub(super) book: Book,
    pub(super) ids: alloc::vec::Vec<(u8, [u8; 6], u32)>,
    /// Records read back that did not decode, left where they were.
    pub(super) refused: usize,
}

impl<K> Store<K> {
    pub fn find(&self, kind: AddrType, addr: &BdAddr) -> Option<&BondRecord> {
        self.book.find(kind, addr)
    }

    pub fn book(&self) -> &Book {
        &self.book
    }

    /// How many records read back at `open` were refused.
    pub fn refused(&self) -> usize {
        self.refused
    }

    pub fn keys(&mut self) -> &mut K {
        &mut self.keys
    }

    /// Where the records are kept, as a restarted capsule would open them.
    pub fn into_keys(self) -> K {
        self.keys
    }

    pub(super) fn id_of(&self, kind: u8, identity: &[u8; 6]) -> Option<usize> {
        self.ids.iter().position(|(k, id, _)| *k == kind && id == identity)
    }
}
