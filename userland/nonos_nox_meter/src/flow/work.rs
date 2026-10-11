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

//! What a job is for, the contracts it uses, and what it asks settle.

use alloc::vec::Vec;

use nonos_payment_proto::{Close, OpenStart, TopUp};

use crate::open::TxRequest;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Work {
    Open(OpenStart),
    TopUp(TopUp),
    Close(Close),
}

impl Work {
    pub fn chain_id(&self) -> u64 {
        match self {
            Work::Open(s) => s.chain_id,
            Work::TopUp(t) => t.chain_id,
            Work::Close(c) => c.chain_id,
        }
    }
}

/// The config's NOX and Settlement on the job's chain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Contracts {
    pub nox: [u8; 20],
    pub settlement: [u8; 20],
}

/// The next request to settle: Settlement's GRACE() (READ view 16), the
/// meter's GAS, or a BROADCAST of one signed transaction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ask {
    Grace,
    Gas,
    Broadcast(Vec<u8>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Next {
    Ask(Ask),
    /// Sign these with the meter's key, then hand the raw transactions back.
    Sign(Vec<TxRequest>),
    /// The job's answer: each broadcast transaction's hash, in order.
    Done(Vec<u8>),
}
