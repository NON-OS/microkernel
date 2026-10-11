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

//! A ChannelOpened log (plan/24) as Opened: channelId, payer and payee are
//! its indexed topics; listingId, amount, expiry, openNonce and reclaimTo its
//! five data words. Every word is read strictly: an amount past u128, an
//! expiry or nonce past u64, or an address word with high bytes is no event.

use crate::opened::Opened;

pub const CHANNEL_OPENED: &str =
    "ChannelOpened(bytes32,address,address,bytes32,uint256,uint64,uint64,address)";
/// keccak256(CHANNEL_OPENED), pinned by a test.
pub const CHANNEL_OPENED_TOPIC: [u8; 32] = [
    0xe7, 0xd5, 0x60, 0xb3, 0x1b, 0xe3, 0x48, 0xa5, 0xfd, 0xf3, 0xdb, 0x55, 0x47, 0x82, 0xa5, 0xe8,
    0x07, 0xf6, 0x52, 0xa7, 0x36, 0x39, 0x8c, 0x62, 0xc8, 0x84, 0xfb, 0xc0, 0xab, 0x41, 0xc3, 0x69,
];

fn addr(w: &[u8]) -> Option<[u8; 20]> {
    w[..12].iter().all(|b| *b == 0).then(|| w[12..32].try_into().ok())?
}

fn num(w: &[u8], bytes: usize) -> Option<u128> {
    let high = 32 - bytes;
    w[..high]
        .iter()
        .all(|b| *b == 0)
        .then(|| w[high..32].iter().fold(0u128, |v, b| v << 8 | *b as u128))
}

impl Opened {
    pub fn from_log(topics: &[[u8; 32]], data: &[u8]) -> Option<Opened> {
        let [t0, channel_id, payer, payee] = topics else { return None };
        if *t0 != CHANNEL_OPENED_TOPIC || data.len() != 5 * 32 {
            return None;
        }
        let w = |i: usize| &data[32 * i..32 * i + 32];
        Some(Opened {
            channel_id: *channel_id,
            payer: addr(payer)?,
            payee: addr(payee)?,
            listing_id: w(0).try_into().ok()?,
            amount: num(w(1), 16)?,
            expiry: num(w(2), 8)? as u64,
            open_nonce: num(w(3), 8)? as u64,
            reclaim_to: addr(w(4))?,
        })
    }
}
