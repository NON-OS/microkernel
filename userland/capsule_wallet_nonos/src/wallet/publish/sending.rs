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

/*
 * A publisher transaction under way (B8): the call, its send, its step,
 * the calls after it, each reviewed and signed, and when settle is next
 * asked for a read that repeats.
 */

use alloc::vec::Vec;

use nonos_settle_proto::{Quote, OP_BROADCAST, OP_GAS, OP_INTENT, OP_READ};

use crate::wallet::intent::{IntentStep, Publisher, PublisherSend};

pub struct Sending {
    pub call: Publisher,
    pub send: PublisherSend,
    pub step: IntentStep,
    pub queue: Vec<Publisher>,
    pub next_ms: u64,
    /// Sign was pressed, armed; the chain's pending nonce is read first.
    pub pressed: bool,
    /// What the keyring signed, broadcast next.
    pub raw: Option<Vec<u8>>,
    /// The quote it was signed at and its own hash, for a broadcast unsure.
    pub signed: Option<(Quote, [u8; 32])>,
    /// BROADCAST's answer when it names another hash: shown, never followed.
    pub echo: Option<[u8; 32]>,
}

impl Sending {
    pub fn new(call: Publisher, send: PublisherSend, step: IntentStep, q: Vec<Publisher>) -> Self {
        let (raw, signed, echo) = (None, None, None);
        Sending { call, send, step, queue: q, next_ms: 0, pressed: false, raw, signed, echo }
    }
}

/// The one settle request the publisher screen has outstanding.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Asked {
    Intent,
    Gas,
    Broadcast,
    /// The sent call's own view, read until it lands.
    Landed,
    /// One of the views that grey the Bond's rows.
    View(u16),
}

impl Asked {
    pub fn op(self) -> u16 {
        match self {
            Asked::Intent => OP_INTENT,
            Asked::Gas => OP_GAS,
            Asked::Broadcast => OP_BROADCAST,
            Asked::Landed | Asked::View(_) => OP_READ,
        }
    }
}
