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

//! SHA-256 in the capsule, for inputs the kernel syscall will not carry.

mod compress;
mod finish;
mod round_constants;
mod update;

/*
 * `types` is public and the `Sha256` handle is not re-exported here. The capsule
 * only ever needs the one shot `digest`, so a re-export would be an unused name;
 * the streaming handle stays reachable for the proofs, which drive update and
 * finish directly to check that a split input hashes the same as a whole one.
 */
pub mod types;

pub use finish::digest;
