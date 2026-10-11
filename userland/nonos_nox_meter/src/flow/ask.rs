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

//! What a job asks settle, what it does next, and the contracts it uses.

use alloc::vec::Vec;

use crate::open::TxRequest;

/// The config's contracts on the job's chain; one the job does not need is
/// zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Contracts {
    pub nox: [u8; 20],
    pub settlement: [u8; 20],
    pub pool: [u8; 20],
}

/// The next request to settle: Settlement's GRACE() (READ view 16), the
/// pool's claimable(NOX, meter) (READ view 60), the meter's GAS, or a
/// BROADCAST of one signed transaction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ask {
    Grace,
    Claimable,
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
