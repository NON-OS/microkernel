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

//! What a job hands back once it is finished.

use alloc::vec::Vec;

use super::super::read_snapshot::Snapshot;
use super::super::status::NetStatus;

/// What one reading of the chain said of the payments on their way.
pub struct Followed {
    /// Each payment's receipt: its block and whether it succeeded, or None.
    pub receipts: Vec<([u8; 32], Option<(u64, bool)>)>,
    pub head: u64,
    pub latest: u64,
    pub pending: u64,
}

/// A job's result, once it has one.
pub enum Finished {
    Probe(NetStatus),
    /// None when the request did not complete, so the link is down.
    Snapshot {
        snap: Option<Snapshot>,
        address: [u8; 20],
        chain: u64,
        /// The host that answered, or failed to.
        host: &'static str,
    },
    /// None when the reading did not come whole.
    Follow {
        followed: Option<Followed>,
        address: [u8; 20],
        chain: u64,
    },
}
