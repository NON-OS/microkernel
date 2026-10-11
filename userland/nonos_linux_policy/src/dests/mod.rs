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

//! The destinations a program may reach, whose canonical bytes the policy's
//! net_dests_hash names (plan/30). A zero hash names no list: any route over
//! the anonymity network.
//!
//! A list is a u16 big-endian count, 1 to 64, then that many entries in
//! strictly increasing order of their bytes, and nothing after. An entry is
//! kind (4, 6, or b'N' for a name), prefix length, first and last port (u16
//! big-endian, inclusive, the first at least 1), the body's length, then the
//! body: an address in network order with its host bits clear, or a name.

mod address;
mod allows;
mod entry;
mod error;
mod list;
mod name;

pub use allows::{allows, Target};
pub use entry::{Dest, Kind};
pub use error::DestsError;
pub use list::{check, dests_hash, entries, DOMAIN, MAX_ENTRIES};
