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

use super::exit_rule::Seen;

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
    /// The owner's STATUS has been answered with the failure.
    pub told: bool,
    /// When it failed or its owner last sent a frame, whichever is later.
    pub asker_ms: u64,
}

impl Job {
    pub fn new(id: u32, owner: u32) -> Job {
        let stage = Stage::Queued;
        Job {
            id,
            owner,
            stage,
            status: 0,
            out: Vec::new(),
            fetched: false,
            told: false,
            asker_ms: 0,
        }
    }

    /// What the exit rule reads of it.
    pub fn seen(&self) -> Seen {
        let (stage, fetched, told, asker_ms) = (self.stage, self.fetched, self.told, self.asker_ms);
        Seen { stage, fetched, told, asker_ms }
    }

    pub fn total(&self) -> u32 {
        if self.stage == Stage::Done {
            self.out.len() as u32
        } else {
            0
        }
    }
}
