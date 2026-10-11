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

//! ORDER_DELIVERED (op 10, plan/21, plan/29), from the wallet or the
//! installer; a job (op 15). Request: orderId[32] | K[32]. Result: 0 |
//! content_len u64 once every chunk checked (op 18 then serves the
//! plaintext), or 1 | chunk index u64 | dispute()'s calldata, the rest.

use alloc::vec::Vec;

use crate::cursor::Cursor;
pub use crate::order_delivered_made::DeliveredMade;

pub const DELIVERED_ASK_LEN: usize = 64;

/// K is the content's key (rule 5): its Debug names it redacted, it is not
/// Copy, and dropping the ask wipes it.
#[derive(Clone, PartialEq, Eq)]
pub struct DeliveredAsk {
    pub order_id: [u8; 32],
    pub key: [u8; 32],
}

impl DeliveredAsk {
    /// K zeroed, as dropping the ask does.
    pub fn wipe(&mut self) {
        self.key.fill(0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl Drop for DeliveredAsk {
    fn drop(&mut self) {
        self.wipe();
    }
}

impl core::fmt::Debug for DeliveredAsk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DeliveredAsk")
            .field("order_id", &self.order_id)
            .field("key", &"redacted")
            .finish()
    }
}

impl DeliveredAsk {
    pub fn encode(&self) -> Vec<u8> {
        [self.order_id, self.key].concat()
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (order_id, key) = (r.fixed()?, r.fixed()?);
        r.end(DeliveredAsk { order_id, key })
    }
}
