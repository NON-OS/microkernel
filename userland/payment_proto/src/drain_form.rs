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

//! DRAIN (op 3), for settle, in two forms told apart by the first byte
//! (plan/21, v1.4.7):
//!
//!   LIST 0 | from u8
//!        reply: Drain, the newest signed voucher of each channel
//!   SIGN 1 | chain_id u64 | nonce u64 | gas_limit u64 | max_fee u128
//!        | max_priority u128 | n u8 | n x channel_id[32]
//!        reply: the raw signed settle() transaction, 0x02 then its RLP
//!
//! For SIGN the meter builds settle(vouchers, sigs) from its own newest
//! vouchers of the named channels; settle supplies only what it quotes.

use crate::bytes::len_u8;
use alloc::vec::Vec;

pub const DRAIN_LIST: u8 = 0;
pub const DRAIN_SIGN: u8 = 1;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DrainSign {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_limit: u64,
    pub max_fee: u128,
    pub max_priority: u128,
    /// At most DRAIN_ROWS_MAX, none twice.
    pub channels: Vec<[u8; 32]>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DrainRequest {
    List { from: u8 },
    Sign(DrainSign),
}

impl DrainRequest {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            DrainRequest::List { from } => alloc::vec![DRAIN_LIST, *from],
            DrainRequest::Sign(s) => {
                let mut out = alloc::vec![DRAIN_SIGN];
                for n in [s.chain_id, s.nonce, s.gas_limit] {
                    out.extend_from_slice(&n.to_le_bytes());
                }
                out.extend_from_slice(&s.max_fee.to_le_bytes());
                out.extend_from_slice(&s.max_priority.to_le_bytes());
                out.push(len_u8(s.channels.len()));
                s.channels.iter().for_each(|c| out.extend_from_slice(c));
                out
            }
        }
    }
}
