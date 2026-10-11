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

//! orderOf as the dispute reads it (plan/24): its state and deliveredAt,
//! and the window's end from the order's own disputeWindow.

use nonos_payment_proto::OrderMade;

pub const DELIVERED: u8 = 2;
/// The terms' disputeWindow word (plan/22's OrderTerms).
const WINDOW_WORD: usize = 7;

pub(super) fn tail_u64(w: &[u8; 32]) -> Option<u64> {
    let (high, low) = w.split_at(24);
    high.iter().all(|b| *b == 0).then(|| u64::from_be_bytes(low.try_into().unwrap_or([0; 8])))
}

/// orderOf's five words, (state, funded, extra, fundedAt, deliveredAt),
/// read as its state and deliveredAt.
pub fn order_of(words: &[[u8; 32]]) -> Option<(u8, u64)> {
    let [state, _, _, _, at] = words else { return None };
    Some((u8::try_from(tail_u64(state)?).ok()?, tail_u64(at)?))
}

/// When the window closes: deliveredAt plus the order's own disputeWindow.
pub fn window_end(m: &OrderMade, delivered_at: u64) -> Option<u64> {
    delivered_at.checked_add(tail_u64(&m.terms[WINDOW_WORD])?)
}
