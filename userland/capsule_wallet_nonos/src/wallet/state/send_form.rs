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
//! The send form: which field has focus, the stage it is at, what is typed
//! and the payment its review shows.

use crate::wallet::num::Amount;
use crate::wallet::send::Draft;

pub struct SendForm {
    pub focus: u8,
    /// Form, review or sent (`send::STAGE_*`).
    pub stage: u8,
    /// The payment the review shows and the confirm signs.
    pub draft: Option<Draft>,
    pub to_hex: [u8; 40],
    pub to_len: usize,
    /// What to send, typed at chain precision on the shared keypad. It replaced
    /// a `u32` of thousandths of an ether, which could not express a figure
    /// smaller than 0.001 and had no decimal point at all.
    pub amount: Amount,
    pub nonce: u64,
    /// The amount is everything held, the fee left out of an ETH payment at
    /// the fees the review reads.
    pub all: bool,
    /// Which asset the send screen transfers: 0 = ETH, 1 = NOX.
    pub token: u8,
}

impl SendForm {
    pub fn new() -> Self {
        Self {
            focus: super::types::SEND_FIELD_TO,
            stage: 0,
            draft: None,
            to_hex: [0; 40],
            to_len: 0,
            amount: Amount::new(),
            nonce: 0,
            all: false,
            token: 0,
        }
    }
}
