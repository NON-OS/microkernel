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

//! DELIVER, op 5 (plan/15): a supervisor hands over what it served; the
//! prover builds every digest itself. The reply is a job, as PROVE's.
//!
//!   listing_id[32] | policy[256] | nonce[32] | order_id[32] | channel_id[32]
//!   | from u64 | to u64 | request_len u32 | request | response

use alloc::vec::Vec;

use super::prove::GET_MAX;

pub const DELIVER: u16 = 5;
pub const DELIVER_HEAD: usize = 32 + 256 + 32 * 3 + 8 + 8 + 4;
/// Request and response together, so a DELIVER frame is no larger than a GET.
pub const DELIVER_DATA_MAX: usize = GET_MAX - DELIVER_HEAD;

#[derive(Clone, Copy)]
pub struct DeliverRequest<'a> {
    pub listing_id: &'a [u8; 32],
    pub policy: &'a [u8; 256],
    pub nonce: &'a [u8; 32],
    pub order_id: &'a [u8; 32],
    pub channel_id: &'a [u8; 32],
    pub from: u64,
    pub to: u64,
    pub request: &'a [u8],
    pub response: &'a [u8],
}

impl<'a> DeliverRequest<'a> {
    pub fn encode(&self) -> Vec<u8> {
        let ids = [&self.nonce[..], self.order_id, self.channel_id];
        let span = [self.from.to_le_bytes(), self.to.to_le_bytes()].concat();
        let n = u32::try_from(self.request.len()).unwrap_or(u32::MAX).to_le_bytes(); // decode refuses
        let head = [&self.listing_id[..], self.policy, &ids.concat(), &span, &n].concat();
        [&head[..], self.request, self.response].concat()
    }

    /// `None` for a short head, data past DELIVER_DATA_MAX, or a bad length.
    pub fn decode(b: &'a [u8]) -> Option<DeliverRequest<'a>> {
        let at = |i: usize, n: usize| b.get(i..i + n);
        let u64_at = |i: usize| Some(u64::from_le_bytes(at(i, 8)?.try_into().ok()?));
        let len = u32::from_le_bytes(at(DELIVER_HEAD - 4, 4)?.try_into().ok()?) as usize;
        let data = b.get(DELIVER_HEAD..)?;
        if data.len() > DELIVER_DATA_MAX || len > data.len() {
            return None;
        }
        Some(DeliverRequest {
            listing_id: at(0, 32)?.try_into().ok()?,
            policy: at(32, 256)?.try_into().ok()?,
            nonce: at(288, 32)?.try_into().ok()?,
            order_id: at(320, 32)?.try_into().ok()?,
            channel_id: at(352, 32)?.try_into().ok()?,
            from: u64_at(384)?,
            to: u64_at(392)?,
            request: &data[..len],
            response: &data[len..],
        })
    }
}
