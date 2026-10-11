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

use super::body::{Delivery, DELIVERY_BYTES};
use crate::words::{b32_at, u64_at};

impl Delivery {
    pub fn encode(&self) -> [u8; DELIVERY_BYTES] {
        let mut b = [0u8; DELIVERY_BYTES];
        let words = [self.record_id, self.service_meas, self.req, self.rsp, self.order_ref];
        for (i, w) in words.iter().enumerate() {
            b[32 * i..32 * i + 32].copy_from_slice(w);
        }
        b[160..168].copy_from_slice(&self.from.to_le_bytes());
        b[168..176].copy_from_slice(&self.to.to_le_bytes());
        b[176..].copy_from_slice(&self.channel_ref);
        b
    }

    pub fn decode(b: &[u8]) -> Option<Delivery> {
        if b.len() != DELIVERY_BYTES {
            return None;
        }
        Some(Delivery {
            record_id: b32_at(b, 0)?,
            service_meas: b32_at(b, 32)?,
            req: b32_at(b, 64)?,
            rsp: b32_at(b, 96)?,
            order_ref: b32_at(b, 128)?,
            from: u64_at(b, 160)?,
            to: u64_at(b, 168)?,
            channel_ref: b32_at(b, 176)?,
        })
    }
}
