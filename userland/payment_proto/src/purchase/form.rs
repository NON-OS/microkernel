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

//! PURCHASE's four forms (op 19, the plan, v1.4.42), and READY's body: the
//! names again with the orderId ORDER_NEW answered, held for the store so
//! it can watch ORDER_STATUS for that order.

use alloc::vec::Vec;

use crate::purchase::{Names, PURCHASE_POST, PURCHASE_READY, PURCHASE_TAKE, PURCHASE_TAKE_READY};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ready {
    pub names: Names,
    pub order_id: [u8; 32],
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Purchase {
    Post(Names),
    Take,
    Ready(Ready),
    TakeReady,
}

impl Ready {
    /// As TAKE_READY answers it, and as READY carries it after its byte.
    pub fn encode(&self) -> Option<Vec<u8>> {
        Some([&self.names.encode()?[..], &self.order_id].concat())
    }

    pub fn post(&self) -> Option<Vec<u8>> {
        Some([&[PURCHASE_READY][..], &self.encode()?].concat())
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let at = b.len().checked_sub(32)?;
        let names = Names::parse(&b[..at])?;
        Some(Ready { names, order_id: b[at..].try_into().ok()? })
    }
}

/// A PURCHASE body, or None for any other first byte, or a TAKE carrying
/// anything after it.
pub fn parse_purchase(b: &[u8]) -> Option<Purchase> {
    match b.split_first()? {
        (&PURCHASE_POST, rest) => Names::parse(rest).map(Purchase::Post),
        (&PURCHASE_TAKE, []) => Some(Purchase::Take),
        (&PURCHASE_READY, rest) => Ready::parse(rest).map(Purchase::Ready),
        (&PURCHASE_TAKE_READY, []) => Some(Purchase::TakeReady),
        _ => None,
    }
}
