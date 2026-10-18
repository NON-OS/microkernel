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

//! Reading the bonds back. A record that does not decode, or that would
//! not fit in the book, is left where it is and counted: this capsule does
//! not destroy what it cannot read.

use super::types::Store;
use crate::book::Book;
use crate::error::BondError;
use crate::keys::{Keys, KIND_BOND};
use crate::record::BondRecord;
use crate::wipe::wipe;
use alloc::vec::Vec;

impl<K: Keys> Store<K> {
    pub fn open(mut keys: K) -> Result<Self, BondError> {
        let (mut book, mut ids, mut refused) = (Book::default(), Vec::new(), 0);
        for id in keys.list(KIND_BOND)? {
            let mut bytes = keys.retrieve(id)?;
            let read = BondRecord::decode(&bytes);
            wipe(&mut bytes);
            let Ok(record) = read else {
                refused += 1;
                continue;
            };
            let identity = (record.identity_type, record.identity, id);
            match book.insert(record) {
                Ok(()) => ids.push(identity),
                Err(_) => refused += 1,
            }
        }
        Ok(Self { keys, book, ids, refused })
    }
}
