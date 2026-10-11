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

//! DRAIN's reply: the newest signed voucher of each channel, for settle to
//! submit. The request is one byte, the first signed voucher wanted; reading
//! takes nothing away, since only the newest voucher of a channel is worth
//! settling and it stays the newest until the next charge. Little-endian:
//!
//!   total u8 | rows u8 | rows x (channel_id[32] | payer[20] | payee[20]
//!   | listing_id[32] | unit[32] | cumulative u128 | seq u64 | expiry u64
//!   | usage_root[32] | sig[65])

use alloc::vec::Vec;

use crate::state::MeterState;
use crate::voucher::Voucher;

pub const DRAIN_ROWS_MAX: usize = 12;
pub const VOUCHER_RECORD_LEN: usize = 32 + 20 + 20 + 32 + 32 + 16 + 8 + 8 + 32 + 65;

/// `payer` is the meter's own address.
pub fn drain_reply(s: &MeterState, payer: &[u8; 20], from: usize) -> Vec<u8> {
    let signed = || s.channels.iter().filter_map(|c| c.signed(payer));
    let mut out = Vec::with_capacity(2 + DRAIN_ROWS_MAX * VOUCHER_RECORD_LEN);
    out.push(signed().count() as u8);
    out.push(signed().skip(from).take(DRAIN_ROWS_MAX).count() as u8);
    for (v, sig) in signed().skip(from).take(DRAIN_ROWS_MAX) {
        voucher_record(&mut out, &v, &sig);
    }
    out
}

/// One signed voucher, VOUCHER_RECORD_LEN bytes: DRAIN's row and PAY's reply.
pub fn voucher_record(out: &mut Vec<u8>, v: &Voucher, sig: &[u8; 65]) {
    out.extend_from_slice(&v.channel_id);
    out.extend_from_slice(&v.payer);
    out.extend_from_slice(&v.payee);
    out.extend_from_slice(&v.listing_id);
    out.extend_from_slice(&v.unit);
    out.extend_from_slice(&v.cumulative.to_le_bytes());
    out.extend_from_slice(&v.seq.to_le_bytes());
    out.extend_from_slice(&v.expiry.to_le_bytes());
    out.extend_from_slice(&v.usage_root);
    out.extend_from_slice(sig);
}
