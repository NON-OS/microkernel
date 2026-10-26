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

//! A Linux package as one read-only store object, its pack
//! (docs/design/linux-package-pack.md): one store name per package, however
//! many files it holds. `Pack::parse` holds untrusted bytes to the format's
//! bounds and refuses the whole pack on any fault; `build` writes the one
//! form the parser accepts.

#![no_std]

extern crate alloc;

mod build;
mod entry;
mod format;
mod header;
mod index;
mod kind;
mod layout;
mod lookup;
mod name;
mod parse;
mod refused;
mod verify;
mod write;

pub use build::{build, Source};
pub use entry::Entry;
pub use format::{ENTRY_LEN, HEADER_LEN, MAGIC, MAX_ENTRIES, MAX_PACK, VERSION};
pub use index::Index;
pub use kind::Kind;
pub use parse::Pack;
pub use refused::Refused;
