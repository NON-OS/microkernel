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

//! What fetch holds: one transfer at a time, from its first PUT chunk or
//! its first GET to Held or Failed. A finished transfer gives way to the
//! next; one under way makes every other EBUSY.

use alloc::vec::Vec;

use nonos_proof_abi::wire::fetch::Stage;

pub struct Transfer {
    /// The pid whose PUT chunks fill it; 0 for a download.
    pub owner: u32,
    pub listing_id: [u8; 32],
    /// Known once the last chunk is in, or from the GET.
    pub anchor: [u8; 32],
    pub stage: Stage,
    pub status: i32,
    pub total: u32,
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct Desk {
    pub t: Option<Transfer>,
}

impl Desk {
    /// A PUT left unfinished: its caller went quiet or away.
    pub fn abandon(&mut self) {
        if self.t.as_ref().is_some_and(|t| t.stage == Stage::Receiving) {
            self.t = None;
        }
    }

    pub fn busy(&self) -> bool {
        self.t.as_ref().is_some_and(|t| !matches!(t.stage, Stage::Held | Stage::Failed))
    }
}
