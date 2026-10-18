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

//! OP_OPENED. Request: chain_hash(chain_id, tx_hash). Reply body, the
//! ChannelOpened event the transaction emitted (plan/24): channel_id[32]
//! | payer[20] | payee[20] | listing_id[32] | amount u128 | expiry u64
//! | open_nonce u64 | reclaim_to[20].

use crate::bytes::fixed;

pub const OPENED_LEN: usize = 32 + 20 + 20 + 32 + 16 + 8 + 8 + 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Opened {
    pub channel_id: [u8; 32],
    pub payer: [u8; 20],
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    pub amount: u128,
    pub expiry: u64,
    pub open_nonce: u64,
    pub reclaim_to: [u8; 20],
}

impl Opened {
    pub fn encode(&self) -> [u8; OPENED_LEN] {
        let mut out = [0u8; OPENED_LEN];
        out[..32].copy_from_slice(&self.channel_id);
        out[32..52].copy_from_slice(&self.payer);
        out[52..72].copy_from_slice(&self.payee);
        out[72..104].copy_from_slice(&self.listing_id);
        out[104..120].copy_from_slice(&self.amount.to_le_bytes());
        out[120..128].copy_from_slice(&self.expiry.to_le_bytes());
        out[128..136].copy_from_slice(&self.open_nonce.to_le_bytes());
        out[136..].copy_from_slice(&self.reclaim_to);
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        if b.len() != OPENED_LEN {
            return None;
        }
        Some(Opened {
            channel_id: fixed(b, 0)?,
            payer: fixed(b, 32)?,
            payee: fixed(b, 52)?,
            listing_id: fixed(b, 72)?,
            amount: u128::from_le_bytes(fixed(b, 104)?),
            expiry: u64::from_le_bytes(fixed(b, 120)?),
            open_nonce: u64::from_le_bytes(fixed(b, 128)?),
            reclaim_to: fixed(b, 136)?,
        })
    }
}
