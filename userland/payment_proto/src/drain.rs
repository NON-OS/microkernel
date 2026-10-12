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

//! DRAIN LIST's reply (drain_form.rs has both forms): total u8 | rows u8
//! | rows x VoucherRecord, at most DRAIN_ROWS_MAX.
//! Reading takes nothing away: only a channel's newest voucher is worth
//! settling, and it stays the newest until the next charge.

use alloc::vec::Vec;

use crate::bytes::len_u8;
use crate::cursor::Cursor;
use crate::voucher::{VoucherRecord, DRAIN_ROWS_MAX, VOUCHER_RECORD_LEN};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Drain {
    pub total: u8,
    pub rows: Vec<VoucherRecord>,
}

impl Drain {
    pub fn encode(&self) -> Vec<u8> {
        let rows = self.rows.len().saturating_mul(VOUCHER_RECORD_LEN);
        let mut out = Vec::with_capacity(rows.saturating_add(2));
        out.extend_from_slice(&[self.total, len_u8(self.rows.len())]);
        self.rows.iter().for_each(|v| v.put(&mut out));
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (total, n) = (r.u8()?, r.u8()? as usize);
        (n <= DRAIN_ROWS_MAX).then_some(())?;
        let rows = (0..n).map(|_| VoucherRecord::read(&mut r)).collect::<Option<Vec<_>>>()?;
        r.end(Drain { total, rows })
    }
}

impl VoucherRecord {
    pub(crate) fn read(r: &mut Cursor) -> Option<Self> {
        Some(VoucherRecord {
            channel_id: r.fixed()?,
            payer: r.fixed()?,
            payee: r.fixed()?,
            listing_id: r.fixed()?,
            unit: r.fixed()?,
            cumulative: r.u128()?,
            seq: r.u64()?,
            expiry: r.u64()?,
            usage_root: r.fixed()?,
            sig: r.fixed()?,
        })
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let v = Self::read(&mut r)?;
        r.end(v)
    }
}
