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

//! The tree as bytes: a body naming every linked node, journal records of
//! what each later commit changed, and a slot naming the generation, the
//! body's hash and objects, and each record's object and hash.
//!
//! The body and slot are the broker's only persistent format besides file
//! bytes. Both decode only what they encode, and a decoded body is checked
//! to be a tree before anything uses it: a manifest a cut or a fault left
//! wrong is refused, never half read.

mod check;
mod decode;
mod encode;
mod journal;
mod node_codec;
mod slot;

pub use decode::{decode, finish, parse, Decoded, Parsed};
pub use encode::{encode, first_body, NEXT_ID_AT};
pub use journal::{apply, record, RECORD_NEXT_ID_AT};
pub use node_codec::print;
pub use slot::{body_hash, Record, Slot, JOURNAL_MAX, SLOT_MAX};

/// Object numbers below this are the two slots; chunks and body parts are
/// numbered from it.
pub const FIRST_ID: u64 = 2;

pub(crate) const BODY_MAGIC: [u8; 4] = *b"NUFB";
pub(crate) const SLOT_MAGIC: [u8; 4] = *b"NUFS";
pub(crate) const VERSION: u16 = 1;
