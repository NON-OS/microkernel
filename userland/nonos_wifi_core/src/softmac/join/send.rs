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

//! Putting a join frame on the air. The last frame is resent after mac80211's
//! own quiet interval: 200 ms, or 2 s for SAE, whose commit the AP computes
//! before it answers. A resend is a new frame with the next sequence number
//! and the Retry bit clear (802.11-2020 10.3.2.14.3): an AP that took the
//! first copy discards a retry carrying the same number.

use super::counters::Counters;
use crate::dot11::header::{seq_control, MAC_HEADER_LEN};
use crate::sae::frame::AUTH_ALG_SAE;
use crate::softmac::radio::Radio;

pub const RETX_AFTER_MS: u64 = 200;
pub const RETX_SAE_AFTER_MS: u64 = 2_000;
const FC_RETRY: u8 = 0x08;
const FC0_AUTH: u8 = 0xB0;

pub(super) fn transmit<R: Radio>(r: &mut R, mpdu: &mut [u8], seq: &mut u16, c: &mut Counters) {
    if mpdu.len() >= MAC_HEADER_LEN {
        mpdu[1] &= !FC_RETRY;
        mpdu[22..24].copy_from_slice(&seq_control(*seq).to_le_bytes());
        *seq = seq.wrapping_add(1) & 0x0FFF;
    }
    if r.send(mpdu) {
        c.sent += 1;
    }
}

pub(super) fn resend_after(f: &[u8]) -> u64 {
    let sae = f.len() >= MAC_HEADER_LEN + 2
        && f[0] == FC0_AUTH
        && u16::from_le_bytes([f[MAC_HEADER_LEN], f[MAC_HEADER_LEN + 1]]) == AUTH_ALG_SAE;
    if sae {
        RETX_SAE_AFTER_MS
    } else {
        RETX_AFTER_MS
    }
}
