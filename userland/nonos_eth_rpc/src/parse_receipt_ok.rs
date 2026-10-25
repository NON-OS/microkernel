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

/// Whether the receipt says the transaction succeeded. None is no receipt
/// yet (`"result":null`, a transaction still waiting for a block), an error
/// or an answer without a status: the receipt is read again, never taken
/// as a revert.
pub fn parse_receipt_ok(resp: &[u8]) -> Option<bool> {
    if resp.windows(13).any(|w| w == b"\"result\":null") {
        return None;
    }
    if resp.windows(14).any(|w| w == b"\"status\":\"0x1\"") {
        return Some(true);
    }
    if resp.windows(14).any(|w| w == b"\"status\":\"0x0\"") {
        return Some(false);
    }
    None
}

/// The same, for the receipt of the transaction `hash` only: a receipt that
/// names another transaction is no receipt of this one.
pub fn parse_receipt_for(resp: &[u8], hash: &[u8; 32]) -> Option<bool> {
    let ok = parse_receipt_ok(resp)?;
    let mut want = alloc::vec::Vec::with_capacity(84);
    want.extend_from_slice(b"\"transactionhash\":\"0x");
    for b in hash {
        for n in [b >> 4, b & 15] {
            want.push(b"0123456789abcdef"[n as usize]);
        }
    }
    /* Compared without case: a node may write the hash in capitals. */
    let lower: alloc::vec::Vec<u8> = resp.iter().map(u8::to_ascii_lowercase).collect();
    lower.windows(want.len()).any(|w| w == want.as_slice()).then_some(ok)
}

/// The block a receipt names, from its `blockNumber`. None for no receipt,
/// or one without a block, which is no receipt yet.
pub fn parse_receipt_block(resp: &[u8]) -> Option<u64> {
    const KEY: &[u8] = b"\"blockNumber\":\"0x";
    let at = resp.windows(KEY.len()).position(|w| w == KEY)? + KEY.len();
    let digits = resp.get(at..)?.iter().take_while(|b| b.is_ascii_hexdigit());
    let mut out = 0u64;
    let mut any = false;
    for b in digits {
        let v = (*b as char).to_digit(16)?;
        out = out.checked_mul(16)?.checked_add(u64::from(v))?;
        any = true;
    }
    any.then_some(out)
}
