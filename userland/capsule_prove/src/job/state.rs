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

//! The one job a headless prover holds: who asked, how far it is, and the
//! container once it is made.

use alloc::vec::Vec;

use nonos_proof_abi::wire::Stage;

pub struct Job {
    pub id: u32,
    /// The pid whose PROVE made it; STATUS and GET answer no one else.
    pub owner: u32,
    pub stage: Stage,
    /// Why it failed, once the stage is Failed; 0 otherwise.
    pub status: i32,
    pub out: Vec<u8>,
    /// GET has handed out the container's last byte.
    pub fetched: bool,
}

impl Job {
    pub fn new(id: u32, owner: u32) -> Job {
        Job { id, owner, stage: Stage::Queued, status: 0, out: Vec::new(), fetched: false }
    }

    pub fn total(&self) -> u32 {
        if self.stage == Stage::Done {
            self.out.len() as u32
        } else {
            0
        }
    }
}
