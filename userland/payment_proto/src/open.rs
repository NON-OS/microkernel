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

//! OPEN (op 12), from the wallet, in two forms told apart by the first byte.
//!
//!   START   0 | chain_id u64 | payee[20] | listing_id[32] | unit[32]
//!           | amount u128 | expiry u64 | reclaim_to[20]
//!           reply: approve_tx[32] | open_tx[32]
//!   CONFIRM 1 | the same fields | open_tx[32]
//!           reply: channel_id[32], or EAGAIN while the open is unmined
//!
//! The meter keeps nothing between the two: the wallet holds open_tx, so a
//! reboot between them loses nothing, and CONFIRM may be sent again.

pub const OPEN_START: u8 = 0;
pub const OPEN_CONFIRM: u8 = 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OpenStart {
    pub chain_id: u64,
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    /// The unit the channel's vouchers carry; the event does not name one.
    pub unit: [u8; 32],
    pub amount: u128,
    pub expiry: u64,
    /// A fresh address the wallet derives for this channel: close() pays the
    /// unspent deposit there, so a lost meter key strands nothing.
    pub reclaim_to: [u8; 20],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OpenConfirm {
    pub start: OpenStart,
    pub open_tx: [u8; 32],
}
