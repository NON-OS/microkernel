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

//! What the serving thread keeps while the worker proves, and the container
//! it makes of it once the proofs are in (worker.rs, poll.rs).

use alloc::vec::Vec;

use isolation_stark::Statement;
use nonos_proof_abi::DELIVERY_BYTES;

use super::work::Made;
use crate::market::{container, container2, Sealed};

pub enum Finish {
    /// A PROVE: the kind-1 container of both proofs.
    Market { day: u64, s: Sealed, st: Statement, policy: [u8; 256] },
    /// A DELIVER: the kind-2 container of the device proof alone.
    Delivery { day: u64, s: Sealed, body: [u8; DELIVERY_BYTES] },
}

impl Finish {
    pub fn container(&self, m: &Made) -> Option<Vec<u8>> {
        match self {
            Finish::Market { day, s, st, policy } => {
                container(*day, s, &m.device, &m.isolation, st, policy)
            }
            Finish::Delivery { day, s, body } => container2(*day, s, &m.device, body),
        }
    }
}
