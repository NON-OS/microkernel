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

use nonos_settle_proto::Gas;

use super::ask::{Ask, Contracts, Next};
use super::plans::txs;
use super::work::Work;
use crate::error::MeterError;
use crate::state::MeterState;

/// The next step, or why the job ends here.
pub type Step = Result<Next, MeterError>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Flow {
    pub work: Work,
    read: Option<u128>,
    raws: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    hashes: alloc::vec::Vec<u8>,
}

impl Flow {
    pub fn begin(work: Work) -> (Flow, Next) {
        let first = Next::Ask(work.first());
        (Flow { work, read: None, raws: alloc::vec![], hashes: alloc::vec![] }, first)
    }

    /// The job's one read: GRACE to close, what is claimable to claim.
    pub fn on_read(&mut self, value: u128) -> Step {
        let nothing = matches!(self.work, Work::Claim { .. }) && value == 0;
        self.read = Some(value);
        (!nothing).then_some(Next::Ask(Ask::Gas)).ok_or(MeterError::NothingOwed)
    }

    /// What to sign, from GAS: `at` is the config's contracts and the time now.
    pub fn on_gas(&mut self, g: &Gas, s: &MeterState, at: (&Contracts, u64)) -> Step {
        txs(&self.work, self.read, g, s, at.0, at.1).map(Next::Sign)
    }

    /// The signed transactions, in Sign's order.
    pub fn on_signed(&mut self, raws: alloc::vec::Vec<alloc::vec::Vec<u8>>) -> Next {
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
