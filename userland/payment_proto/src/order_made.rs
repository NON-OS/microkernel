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

//! ORDER_NEW's result: orderId[32] | depositAddress[20] | the 14 term words.

use alloc::vec::Vec;

use crate::cursor::Cursor;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OrderMade {
    pub order_id: [u8; 32],
    pub deposit: [u8; 20],
    pub terms: [[u8; 32]; 14],
}

impl OrderMade {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = [&self.order_id[..], &self.deposit].concat();
        self.terms.iter().for_each(|w| out.extend_from_slice(w));
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (order_id, deposit) = (r.fixed()?, r.fixed()?);
        let mut terms = [[0u8; 32]; 14];
        for w in terms.iter_mut() {
            *w = r.fixed()?;
        }
        r.end(OrderMade { order_id, deposit, terms })
    }
}
