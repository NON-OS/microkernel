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

//! Reading a DRAIN request: one form, nothing after it, and SIGN naming one
//! to DRAIN_ROWS_MAX channels, none twice.

use alloc::vec::Vec;

use crate::cursor::Cursor;
use crate::drain_form::{DrainRequest, DrainSign, DRAIN_LIST, DRAIN_SIGN};
use crate::voucher::DRAIN_ROWS_MAX;

impl DrainRequest {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        match r.u8()? {
            DRAIN_LIST => {
                let from = r.u8()?;
                r.end(DrainRequest::List { from })
            }
            DRAIN_SIGN => {
                let (chain_id, nonce, gas_limit) = (r.u64()?, r.u64()?, r.u64()?);
                let (max_fee, max_priority, n) = (r.u128()?, r.u128()?, r.u8()? as usize);
                let channels = (0..n).map(|_| r.fixed()).collect::<Option<Vec<[u8; 32]>>>()?;
                let seen =
                    |i: usize, c: &[u8; 32]| channels.get(..i).is_some_and(|b| b.contains(c));
                let unique = channels.iter().enumerate().all(|(i, c)| !seen(i, c));
                ((1..=DRAIN_ROWS_MAX).contains(&n) && unique).then_some(())?;
                let s = DrainSign { chain_id, nonce, gas_limit, max_fee, max_priority, channels };
                r.end(DrainRequest::Sign(s))
            }
            _ => None,
        }
    }
}
