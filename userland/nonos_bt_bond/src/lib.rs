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

//! Bonds for an LE-only host, from the Bluetooth Core Specification 5.4
//! (cited by volume, part and section). A bond is what pairing leaves
//! behind so the peer can be found and the link encrypted again without
//! pairing (Vol 3, Part C, 9.4.4): its LTK, its IRK and its identity
//! address. Every record is fixed in size and versioned, and every octet of
//! one is checked when it is read back, since it may come from storage.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod book;
pub mod error;
pub mod keys;
pub mod local;
pub mod record;
pub mod rotation;
mod store;
mod wipe;

pub use book::{Book, MAX_BONDS};
pub use error::BondError;
pub use keys::{Keys, KeysError, MemoryKeys, KIND_BOND, KIND_LOCAL_IRK, MEMORY_RECORDS};
pub use local::{LocalIrk, LOCAL_LEN};
pub use record::{BondRecord, RECORD_LEN};
pub use rotation::{Rotation, ROTATE_MS};
pub use store::Store;
