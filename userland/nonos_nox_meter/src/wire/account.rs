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

//! ACCOUNT's reply, little-endian. The request is one byte, the first channel
//! wanted; the reply pages through the table ACCOUNT_ROWS_MAX at a time:
//!
//!   flags u8 | address[20] | ed25519[32] | cap u128 | spent u128 | expiry u64
//!   | left u128 | total u8 | rows u8
//!   | rows x (channel_id[32] | chain_id u64 | deposit u128 | settled u128
//!             | cumulative u128 | expiry u64 | frozen u8)
//!
//! flags: READY once the meter's keys are open (until then address and
//! ed25519 are zero), KEPT when this boot persists data. On a boot that keeps
//! nothing the meter changes no state, so its budget stays zero. A frozen
//! channel is one settle reported settled above what this meter signed.

use alloc::vec::Vec;

use crate::state::MeterState;

pub const ACCOUNT_ROWS_MAX: usize = 32;
pub const READY: u8 = 1 << 0;
pub const KEPT: u8 = 1 << 1;
pub const ACCOUNT_ROW_LEN: usize = 32 + 8 + 16 * 3 + 8 + 1;

/// The meter's public identity, once its sealed record is open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Identity {
    pub address: [u8; 20],
    pub ed25519: [u8; 32],
}

pub fn account_reply(
    s: &MeterState,
    who: Option<&Identity>,
    kept: bool,
    now: u64,
    from: usize,
) -> Vec<u8> {
    let rows = s.channels.iter().skip(from).take(ACCOUNT_ROWS_MAX);
    let mut out = Vec::with_capacity(111 + ACCOUNT_ROWS_MAX * ACCOUNT_ROW_LEN);
    out.push(if who.is_some() { READY } else { 0 } | if kept { KEPT } else { 0 });
    out.extend_from_slice(&who.map(|w| w.address).unwrap_or([0; 20]));
    out.extend_from_slice(&who.map(|w| w.ed25519).unwrap_or([0; 32]));
    let b = &s.budget;
    out.extend_from_slice(&b.cap.to_le_bytes());
    out.extend_from_slice(&b.spent.to_le_bytes());
    out.extend_from_slice(&b.expiry.to_le_bytes());
    out.extend_from_slice(&b.left(now).to_le_bytes());
    out.push(s.channels.len() as u8);
    out.push(rows.clone().count() as u8);
    for c in rows {
        out.extend_from_slice(&c.id);
        out.extend_from_slice(&c.chain_id.to_le_bytes());
        out.extend_from_slice(&c.deposit.to_le_bytes());
        out.extend_from_slice(&c.settled.to_le_bytes());
        out.extend_from_slice(&c.cumulative.to_le_bytes());
        out.extend_from_slice(&c.expiry.to_le_bytes());
        out.push(c.frozen as u8);
    }
    out
}
