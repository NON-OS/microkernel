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

//! One program the vendor ships, and the box it runs in.

use crate::layout::LEN;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VendorRow {
    /// Where the program is in the store, as `store:` paths name it.
    pub store_path: &'static str,
    /// BLAKE3 of the program's file, as its policy names it.
    pub program_blake3: [u8; 32],
    /// Its policy, canonical: ids zero, the services and calls policies.txt
    /// gives it.
    pub policy: [u8; LEN],
    /// The destination list its policy's net_dests_hash names, canonical
    /// (crate::dests); empty when the hash is zero.
    pub dests: &'static [u8],
}
