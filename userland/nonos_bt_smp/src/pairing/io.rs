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

//! How a step is built, and the random source pairing draws from.

use super::step::{Outcome, Step, User};
use alloc::vec::Vec;

impl Step {
    pub fn send(pdu: Vec<u8>) -> Self {
        Self { send: alloc::vec![pdu], ..Self::default() }
    }

    pub fn user(user: User) -> Self {
        Self { user: Some(user), ..Self::default() }
    }

    pub fn outcome(outcome: Outcome) -> Self {
        Self { outcome: Some(outcome), ..Self::default() }
    }
}

/// The caller's random source: the system's entropy service in a capsule,
/// a fixed stream in the proofs.
pub trait Random {
    fn fill(&mut self, out: &mut [u8]);
}
