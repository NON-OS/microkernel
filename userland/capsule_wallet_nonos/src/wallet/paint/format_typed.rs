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

use crate::wallet::num::Amount;

use super::format_u128::format_u128;

/// An amount rendered exactly as it was typed.
///
/// Not `format_eth`, which fixes four decimal places: that is right for a
/// balance the chain handed us and wrong for a field the reader is still
/// typing into. Someone entering "0.00005" must see their own keystrokes, not
/// a figure rounded to "0.0000" while the wallet quietly holds a different one.
/// The trailing point is kept while a fraction is open, so the caret sits where
/// the next digit will land.
pub fn format_typed(a: &Amount, out: &mut [u8]) -> usize {
    let places = a.places() as usize;
    let mut digits = [0u8; 40];
    let dn = format_u128(a.raw(), &mut digits);
    let whole_len = dn.saturating_sub(places);

    let mut n = 0;
    if whole_len == 0 {
        n += put(&mut out[n..], b"0");
    } else {
        n += put(&mut out[n..], &digits[..whole_len]);
    }
    if !a.point_started() {
        return n;
    }
    n += put(&mut out[n..], b".");
    // A fraction shorter than its place count is left-padded: "0.05" typed as
    // point, zero, five must not print as "0.5".
    for _ in 0..places.saturating_sub(dn) {
        n += put(&mut out[n..], b"0");
    }
    n + put(&mut out[n..], &digits[whole_len..dn])
}

fn put(dst: &mut [u8], src: &[u8]) -> usize {
    let n = src.len().min(dst.len());
    dst[..n].copy_from_slice(&src[..n]);
    n
}
