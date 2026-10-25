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

use super::cursor::Cursor;
use super::decode_channel::channel;
use super::layout::{CHANNEL_LEN, LISTING_LEN, MAGIC, STATE_LEN, VERSION};
use crate::budget::{Budget, ListingCap, MAX_LISTING_CAPS};
use crate::channel::MAX_CHANNELS;
use crate::error::MeterError;
use crate::state::MeterState;

pub fn decode(b: &[u8]) -> Result<MeterState, MeterError> {
    if b.len() != STATE_LEN {
        return Err(MeterError::Corrupt);
    }
    read(&mut Cursor { b, at: 0 }).ok_or(MeterError::Corrupt)
}

fn read(r: &mut Cursor<'_>) -> Option<MeterState> {
    if r.fixed::<4>()? != MAGIC || r.u16()? != VERSION || r.u16()? != 0 {
        return None;
    }
    let last_settle = match (r.u64()?, r.u64()?) {
        (0, 0) => None,
        (0, _) => return None,
        pair => Some(pair),
    };
    let last_retry = r.u64()?;
    let (cap, spent, expiry) = (r.u128()?, r.u128()?, r.u64()?);
    let n = r.u16()? as usize;
    let mut listings = Vec::with_capacity(n.min(MAX_LISTING_CAPS));
    for _ in 0..n {
        listings.push(ListingCap { listing_id: r.fixed()?, cap: r.u128()?, spent: r.u128()? });
    }
    r.zeros(MAX_LISTING_CAPS.checked_sub(n)? * LISTING_LEN)?;
    let m = r.u16()? as usize;
    let mut channels = Vec::with_capacity(m.min(MAX_CHANNELS));
    for _ in 0..m {
        channels.push(channel(r)?);
    }
    r.zeros(MAX_CHANNELS.checked_sub(m)? * CHANNEL_LEN)?;
    Some(MeterState {
        budget: Budget { cap, spent, expiry, listings },
        channels,
        last_settle,
        last_retry,
    })
}
