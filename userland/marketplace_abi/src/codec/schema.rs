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

//! Which index schemas this build reads and which one it writes.
//!
//! 2: every release carries the hash of the zk trailer binding its own
//! measurement to the enrolled set, and the publisher signature covers it.
//! 3: every entry carries a commerce block and every release the fields that
//! sell it, all under the publisher's v3 signature.
//! 4: every entry carries its logo's BLAKE3 after its description, zero
//! for none, under the operator's index signature.

pub const SCHEMA_V2: u32 = 2;
pub const SCHEMA_V3: u32 = 3;
pub const SCHEMA_V4: u32 = 4;

/// The only schema the encoder writes.
pub const CURRENT_SCHEMA: u32 = SCHEMA_V4;

pub fn readable(schema: u32) -> bool {
    (SCHEMA_V2..=SCHEMA_V4).contains(&schema)
}
