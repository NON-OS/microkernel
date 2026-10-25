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

//! refund(T, refundTo, refundSalt)'s calldata (kind 11, plan/24), and
//! orderOf's state and extra for the refund's checks.

use alloc::vec::Vec;

use nonos_payment_proto::OrderMade;

use super::order_of::tail_u64;
use super::send_step::REFUND;

/// The selector from settle_proto's kinds, T inline, refundTo, refundSalt.
pub fn refund_call(m: &OrderMade, refund_to: &[u8; 20], salt: &[u8; 32]) -> Option<Vec<u8>> {
    let k = nonos_settle_proto::KINDS.iter().find(|k| k.id == REFUND)?;
    let mut out = Vec::from(k.calls[0].1.to_be_bytes());
    m.terms.iter().for_each(|w| out.extend_from_slice(w));
    out.extend_from_slice(&[0; 12]);
    out.extend_from_slice(refund_to);
    out.extend_from_slice(salt);
    Some(out)
}

/// orderOf's five words read as its state and extra (words 0 and 2).
pub fn state_and_extra(words: &[[u8; 32]]) -> Option<(u8, u128)> {
    let [state, _, extra, _, _] = words else { return None };
    let (high, low) = extra.split_at(16);
    let extra = u128::from_be_bytes(low.try_into().ok()?);
    let state = u8::try_from(tail_u64(state)?).ok()?;
    high.iter().all(|b| *b == 0).then_some((state, extra))
}

/// After the broadcast, orderOf until it reads Refunded.
pub fn refunded(words: Option<&[[u8; 32]]>) -> bool {
    words.and_then(state_and_extra).is_some_and(|(state, _)| state == 5)
}
