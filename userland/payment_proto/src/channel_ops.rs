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

//! TOPUP (op 13) and CLOSE (op 14), from the wallet (plan/21, v1.4.7).
//!
//!   TOPUP chain_id u64 | channel_id[32] | amount u128
//!         reply: approve_tx[32] | topup_tx[32]
//!   CLOSE chain_id u64 | channel_id[32]
//!         reply: close_tx[32]; ETIME before expiry plus GRACE
//!
//! The meter signs each with its own key, and settle broadcasts. Anyone may
//! close: the meter's key is a convenience there, never a need.

use alloc::vec::Vec;

use crate::cursor::Cursor;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TopUp {
    pub chain_id: u64,
    pub channel_id: [u8; 32],
    pub amount: u128,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Close {
    pub chain_id: u64,
    pub channel_id: [u8; 32],
}

impl TopUp {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Close { chain_id: self.chain_id, channel_id: self.channel_id }.encode();
        out.extend_from_slice(&self.amount.to_le_bytes());
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (chain_id, channel_id, amount) = (r.u64()?, r.fixed()?, r.u128()?);
        r.end(TopUp { chain_id, channel_id, amount })
    }
}

impl Close {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(56);
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        out.extend_from_slice(&self.channel_id);
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (chain_id, channel_id) = (r.u64()?, r.fixed()?);
        r.end(Close { chain_id, channel_id })
    }
}
