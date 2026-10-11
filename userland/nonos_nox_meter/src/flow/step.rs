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

use super::after::after_reads;
use super::ask::{Ask, Contracts, Next, Step};
use super::plans::txs;
use super::work::Work;
use crate::state::MeterState;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Flow {
    pub work: Work,
    got: Vec<Vec<[u8; 32]>>,
    raws: Vec<Vec<u8>>,
    hashes: Vec<u8>,
}

impl Flow {
    pub fn begin(work: Work) -> (Flow, Next) {
        let first = Next::Ask(work.first());
        (Flow { work, got: Vec::new(), raws: Vec::new(), hashes: Vec::new() }, first)
    }

    /// A read's words, in Work::reads' order: the next read, or what follows.
    pub fn on_view(&mut self, words: Vec<[u8; 32]>) -> Step {
        self.got.push(words);
        match self.work.reads().into_iter().nth(self.got.len()) {
            Some((view, args)) => Ok(Next::Ask(Ask::View { view, args })),
            None => after_reads(&self.work, &self.got),
        }
    }

    /// What to sign, from GAS: `at` is the config's contracts and the time now.
    pub fn on_gas(&mut self, g: &Gas, s: &MeterState, at: (&Contracts, u64)) -> Step {
        txs(&self.work, &self.got, g, s, at.0, at.1).map(Next::Sign)
    }

    /// The signed transactions, in Sign's order.
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
