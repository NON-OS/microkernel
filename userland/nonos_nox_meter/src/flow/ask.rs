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

use crate::error::MeterError;
use crate::escrow::KeptOffer;
use crate::open::TxRequest;
use crate::orders::Order;

/// The config's contracts on the job's chain; one the job does not need is
/// zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Contracts {
    pub nox: [u8; 20],
    pub settlement: [u8; 20],
    pub pool: [u8; 20],
}

/// The next request to settle: a READ of one view with its argument words,
/// the meter's GAS, or a BROADCAST of one signed transaction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ask {
    View {
        view: u16,
        args: Vec<[u8; 32]>,
    },
    Gas,
    Broadcast(Vec<u8>),
    /// A FETCH body for settle (plan/29: FETCH 1 ESCROW, then its PAGEs).
    Fetch(Vec<u8>),
    /// DELIVERED (settle op 12, v1.4.46): this order's key from its log.
    Delivered([u8; 32]),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Next {
    Ask(Ask),
    /// Sign these with the meter's key, then hand the raw transactions back.
    Sign(Vec<TxRequest>),
    /// The job's answer: each broadcast transaction's hash, in order, or an
    /// order's status.
    Done(Vec<u8>),
    /// Drop this order from the table first, then answer (v1.4.26).
    Drop([u8; 32], Vec<u8>),
    /// Seal and keep this order first, then answer with these bytes.
    Keep(alloc::boxed::Box<Order>, Vec<u8>),
    /// Seal and keep this offer first, then answer with these bytes.
    KeepOffer(alloc::boxed::Box<KeptOffer>, Vec<u8>),
    /// Open this order's delivery with this key, as ORDER_DELIVERED does.
    Deliver([u8; 32], [u8; 32]),
}

/// The next step, or why the job ends here.
pub type Step = Result<Next, MeterError>;
