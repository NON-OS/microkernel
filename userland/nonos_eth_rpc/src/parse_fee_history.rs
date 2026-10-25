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

//! The next block's base fee and the tips the last blocks paid, as
//! `eth_feeHistory` reports them (`fees` turns them into a tip and a cap).
//! Ported from the wallet's send/fees.rs, which keeps its own copy.

use alloc::vec::Vec;

/// The next block's base fee and each block's median tip, from one
/// `eth_feeHistory` answer asked with the 50th percentile only.
pub fn parse_fee_history(obj: &[u8]) -> Option<(u128, Vec<u128>)> {
    if obj.windows(7).any(|w| w == b"\"error\"") {
        return None;
    }
    let bases = quantities(array_after(obj, b"\"baseFeePerGas\"")?);
    let next = *bases.last()?;
    let tips = quantities(array_after(obj, b"\"reward\"")?);
    Some((next, tips))
}

/* The bytes of the JSON array that follows `key`, brackets included, with
 * any nested arrays inside it. */
fn array_after<'a>(obj: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let at = obj.windows(key.len()).position(|w| w == key)? + key.len();
    let rest = obj.get(at..)?;
    let open = rest.iter().position(|b| *b == b'[')?;
    let mut depth = 0usize;
    for (i, b) in rest.iter().enumerate().skip(open) {
        match b {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return rest.get(open..=i);
                }
            }
            _ => {}
        }
    }
    None
}

/* Every "0x.." quantity in `text`, in order. One past u128 is no quantity. */
fn quantities(text: &[u8]) -> Vec<u128> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(start) = text.get(i..).and_then(|t| t.windows(3).position(|w| w == b"\"0x")) {
        let digits_at = i + start + 3;
        let digits: Vec<u8> =
            text[digits_at..].iter().take_while(|b| b.is_ascii_hexdigit()).copied().collect();
        let value = digits.iter().try_fold(0u128, |acc, b| {
            acc.checked_mul(16)?.checked_add(u128::from((*b as char).to_digit(16)?))
        });
        if let (Some(v), false) = (value, digits.is_empty()) {
            out.push(v);
        }
        i = digits_at + digits.len();
    }
    out
}
