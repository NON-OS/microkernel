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
use alloc::vec::Vec;

use super::layout::{CHANNEL_LEN, LISTING_LEN, MAGIC, STATE_LEN, VERSION};
use crate::budget::MAX_LISTING_CAPS;
use crate::channel::{Channel, MAX_CHANNELS};
use crate::state::MeterState;

/// The state as its file, or None when it holds more than the file has room
/// for, which a state built through `with_channel` and `absorb` never does.
pub fn encode(s: &MeterState) -> Option<Vec<u8>> {
    let b = &s.budget;
    if b.listings.len() > MAX_LISTING_CAPS || s.channels.len() > MAX_CHANNELS {
        return None;
    }
    let mut out = Vec::with_capacity(STATE_LEN);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    let (chain, nonce) = s.last_settle.unwrap_or((0, 0));
    out.extend_from_slice(&chain.to_le_bytes());
    out.extend_from_slice(&nonce.to_le_bytes());
    out.extend_from_slice(&s.last_retry.to_le_bytes());
    out.extend_from_slice(&b.cap.to_le_bytes());
    out.extend_from_slice(&b.spent.to_le_bytes());
    out.extend_from_slice(&b.expiry.to_le_bytes());
    out.extend_from_slice(&(b.listings.len() as u16).to_le_bytes());
    for l in &b.listings {
        out.extend_from_slice(&l.listing_id);
        out.extend_from_slice(&l.cap.to_le_bytes());
        out.extend_from_slice(&l.spent.to_le_bytes());
    }
    out.resize(out.len() + (MAX_LISTING_CAPS - b.listings.len()) * LISTING_LEN, 0);
    out.extend_from_slice(&(s.channels.len() as u16).to_le_bytes());
    for c in &s.channels {
        channel(&mut out, c);
    }
    out.resize(STATE_LEN, 0);
    Some(out)
}

fn channel(out: &mut Vec<u8>, c: &Channel) {
    let start = out.len();
    out.extend_from_slice(&c.id);
    out.extend_from_slice(&c.payee);
    out.extend_from_slice(&c.listing_id);
    out.extend_from_slice(&c.chain_id.to_le_bytes());
    out.extend_from_slice(&c.open_nonce.to_le_bytes());
    out.extend_from_slice(&c.unit);
    out.extend_from_slice(&c.deposit.to_le_bytes());
    out.extend_from_slice(&c.settled.to_le_bytes());
    out.extend_from_slice(&c.cumulative.to_le_bytes());
    out.extend_from_slice(&c.seq.to_le_bytes());
    out.extend_from_slice(&c.expiry.to_le_bytes());
    out.extend_from_slice(&c.usage_root);
    out.extend_from_slice(&c.sig);
    out.push(c.frozen as u8);
    out.extend_from_slice(&c.carried.to_le_bytes());
    out.extend_from_slice(&c.reclaim_to);
    debug_assert_eq!(out.len() - start, CHANNEL_LEN);
}
