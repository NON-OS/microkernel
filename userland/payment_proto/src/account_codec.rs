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

use crate::account::{Account, AccountRow, ACCOUNT_ROWS_MAX, ACCOUNT_ROW_LEN};
use crate::bytes::len_u8;
use crate::cursor::Cursor;
use crate::cursor_put::flag;

impl Account {
    pub fn encode(&self) -> Vec<u8> {
        let rows = self.rows.len().saturating_mul(ACCOUNT_ROW_LEN);
        let mut out = Vec::with_capacity(rows.saturating_add(79));
        out.push(self.flags);
        out.extend_from_slice(&self.address);
        for v in [self.cap, self.spent] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&self.expiry.to_le_bytes());
        out.extend_from_slice(&self.left.to_le_bytes());
        out.push(self.total);
        out.push(len_u8(self.rows.len()));
        for r in &self.rows {
            out.extend_from_slice(&r.channel_id);
            out.extend_from_slice(&r.listing_id);
            out.extend_from_slice(&r.chain_id.to_le_bytes());
            for v in [r.deposit, r.settled, r.cumulative] {
                out.extend_from_slice(&v.to_le_bytes());
            }
            out.extend_from_slice(&r.expiry.to_le_bytes());
            out.push(r.frozen as u8);
            out.extend_from_slice(&r.reclaim_to);
        }
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (flags, address) = (r.u8()?, r.fixed()?);
        let (cap, spent, expiry, left) = (r.u128()?, r.u128()?, r.u64()?, r.u128()?);
        let (total, n) = (r.u8()?, usize::from(r.u8()?));
        // Fields are read in wire order: a struct literal evaluates in the order written.
        let row = |r: &mut Cursor| -> Option<AccountRow> {
            Some(AccountRow {
                channel_id: r.fixed()?,
                listing_id: r.fixed()?,
                chain_id: r.u64()?,
                deposit: r.u128()?,
                settled: r.u128()?,
                cumulative: r.u128()?,
                expiry: r.u64()?,
                frozen: flag(r.u8()?)?,
                reclaim_to: r.fixed()?,
            })
        };
        let rows = (0..n.min(ACCOUNT_ROWS_MAX)).map(|_| row(&mut r)).collect::<Option<Vec<_>>>()?;
        (n <= ACCOUNT_ROWS_MAX).then_some(())?;
        r.end(Account { flags, address, cap, spent, expiry, left, total, rows })
    }
}
