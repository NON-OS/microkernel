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

//! The two places bonds may be kept, behind the one `Keys` the bond store
//! takes.

use alloc::vec::Vec;

use crate::ring::{Ipc, Keyring};
use nonos_bt_bond::{Keys, KeysError, MemoryKeys};

pub enum Kept {
    Ring(Keyring<Ipc>),
    Memory(MemoryKeys),
}

impl Keys for Kept {
    fn store(&mut self, kind: u8, data: &[u8]) -> Result<u32, KeysError> {
        match self {
            Self::Ring(k) => k.store(kind, data),
            Self::Memory(k) => k.store(kind, data),
        }
    }
    fn retrieve(&mut self, id: u32) -> Result<Vec<u8>, KeysError> {
        match self {
            Self::Ring(k) => k.retrieve(id),
            Self::Memory(k) => k.retrieve(id),
        }
    }
    fn delete(&mut self, id: u32) -> Result<(), KeysError> {
        match self {
            Self::Ring(k) => k.delete(id),
            Self::Memory(k) => k.delete(id),
        }
    }
    fn list(&mut self, kind: u8) -> Result<Vec<u32>, KeysError> {
        match self {
            Self::Ring(k) => k.list(kind),
            Self::Memory(k) => k.list(kind),
        }
    }
}
