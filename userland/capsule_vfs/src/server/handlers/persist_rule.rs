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

//! What a persist may do to the disk, pure so the host proofs can walk every
//! case. `persist_gate` asks the kernel and hands the answer here.

/// How a persist may reach the disk this boot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keep {
    /// The person chose to keep data: the record is written as it is.
    Record,
    /// An amnesic boot asked to clear a record: zeros may only take the place
    /// of one the disk already holds, in place, and nothing new is added
    /// (`blk::store_withdraw::withdraw`).
    Withdraw,
}

/// `persistent` is the person's choice at setup, as the kernel holds it;
/// before the policy service has told it, `false`. On a boot that keeps nothing only zeros
/// pass, an empty payload among them, and only as a withdrawal; anything
/// else is refused.
pub fn keep(persistent: bool, data: &[u8]) -> Option<Keep> {
    if persistent {
        Some(Keep::Record)
    } else if data.iter().all(|b| *b == 0) {
        Some(Keep::Withdraw)
    } else {
        None
    }
}
