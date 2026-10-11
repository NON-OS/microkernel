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

//! The flow of one job: each answer moves it on by exactly one step.

use alloc::vec::Vec;

use nonos_settle_proto::Gas;

use super::plans::txs;
use super::work::{Ask, Contracts, Next, Work};
use crate::error::MeterError;
use crate::state::MeterState;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Flow {
    pub work: Work,
    grace: Option<u64>,
    raws: Vec<Vec<u8>>,
    hashes: Vec<u8>,
}

impl Flow {
    pub fn begin(work: Work) -> (Flow, Next) {
        let first = if matches!(work, Work::Close(_)) { Ask::Grace } else { Ask::Gas };
        (Flow { work, grace: None, raws: Vec::new(), hashes: Vec::new() }, Next::Ask(first))
    }

    pub fn on_grace(&mut self, grace: u64) -> Next {
        self.grace = Some(grace);
        Next::Ask(Ask::Gas)
    }

    /// What to sign, from GAS: `at` is the config's contracts and the time now.
    pub fn on_gas(
        &mut self,
        g: &Gas,
        s: &MeterState,
        at: (&Contracts, u64),
    ) -> Result<Next, MeterError> {
        txs(&self.work, self.grace, g, s, at.0, at.1).map(Next::Sign)
    }

    /// The signed transactions, in the order Sign gave them.
    pub fn on_signed(&mut self, raws: Vec<Vec<u8>>) -> Next {
        self.raws = raws;
        self.raws.reverse();
        self.next_broadcast()
    }

    pub fn on_hash(&mut self, h: &[u8; 32]) -> Next {
        self.hashes.extend_from_slice(h);
        self.next_broadcast()
    }

    fn next_broadcast(&mut self) -> Next {
        match self.raws.pop() {
            Some(raw) => Next::Ask(Ask::Broadcast(raw)),
            None => Next::Done(core::mem::take(&mut self.hashes)),
        }
    }
}
