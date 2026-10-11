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

//! SIGN_FROM_FRESH (op 17, plan/27, v1.4.51): a refund or a dispute signed
//! by a fresh address the shield handed out. Fields, one to a line:
//! address, chain_id, nonce, gas, max_fee, max_priority, to, value, data;
//! addresses and data as 0x hex, numbers in decimal. A job: its RESULT is
//! `raw` and `hash`, both 0x hex, or `why` when refused.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::hexes::{hex, unhex};

pub const OP_SIGN_FROM_FRESH: u16 = 17;
/// OP_FRESH_ADDRESS's modes: an address no order shares, and their list (v1.4.52).
pub const FRESH_ONCE: &str = "1";
pub const FRESH_HANDED: &str = "2";

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FreshAsk {
    pub address: [u8; 20],
    pub chain_id: u64,
    pub nonce: u64,
    pub gas: u64,
    pub max_fee: u128,
    pub max_priority: u128,
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
}

impl FreshAsk {
    pub fn fields(&self) -> Vec<String> {
        let n = |v: u128| format!("{v}");
        let (chain, nonce, gas) = (self.chain_id as u128, self.nonce as u128, self.gas as u128);
        let (fee, tip) = (n(self.max_fee), n(self.max_priority));
        Vec::from([hex(&self.address), n(chain), n(nonce), n(gas), fee, tip])
            .into_iter()
            .chain([hex(&self.to), n(self.value), hex(&self.data)])
            .collect()
    }

    pub fn parse(f: &[&str]) -> Option<FreshAsk> {
        let [address, chain_id, nonce, gas, max_fee, max_priority, to, value, data] = f else {
            return None;
        };
        let addr = |s: &str| <[u8; 20]>::try_from(unhex(s)?).ok();
        Some(FreshAsk {
            address: addr(address)?,
            chain_id: chain_id.parse().ok()?,
            nonce: nonce.parse().ok()?,
            gas: gas.parse().ok()?,
            max_fee: max_fee.parse().ok()?,
            max_priority: max_priority.parse().ok()?,
            to: addr(to)?,
            value: value.parse().ok()?,
            data: unhex(data)?,
        })
    }
}
