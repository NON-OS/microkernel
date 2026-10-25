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

//! Signed vouchers as payment_proto's records: PAY's reply and DRAIN's rows.

use nonos_payment_proto::{Drain, VoucherRecord, DRAIN_ROWS_MAX};

use crate::state::MeterState;
use crate::voucher::Voucher;

pub fn record(v: &Voucher, sig: &[u8; 65]) -> VoucherRecord {
    VoucherRecord {
        channel_id: v.channel_id,
        payer: v.payer,
        payee: v.payee,
        listing_id: v.listing_id,
        unit: v.unit,
        cumulative: v.cumulative,
        seq: v.seq,
        expiry: v.expiry,
        usage_root: v.usage_root,
        sig: *sig,
    }
}

/// The newest signed voucher of each channel, a page from `from`. `payer`
/// is the meter's own address.
pub fn drain(s: &MeterState, payer: &[u8; 20], from: usize) -> Drain {
    let signed = || s.channels.iter().filter_map(|c| c.signed(payer));
    Drain {
        total: signed().count() as u8,
        rows: signed().skip(from).take(DRAIN_ROWS_MAX).map(|(v, sig)| record(&v, &sig)).collect(),
    }
}
