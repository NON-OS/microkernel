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

//! Connection lists (HDA 1.0a section 7.3.3.3).
//!
//! The driver never read one: it took the first DAC and the first pin it met
//! and hoped they were wired together. They usually are not. On a Realtek
//! ALC236 the first pin is 0x12, a digital microphone input, and the
//! speaker at 0x14 reaches its DAC only through a connection the codec
//! describes here.
//!
//! The list comes back four short entries (8 bits) or two long ones (16
//! bits) per GET_CONNECT_LIST, by the long-form bit 7 of the length
//! parameter. An entry with its top bit set is a range: every node from the
//! entry before it up to this one is connected (Linux
//! `snd_hda_get_raw_connections`). Connection indices, the ones
//! SET_CONNECT_SEL and the input amps take, count the expanded list.

use super::widget::MAX_CONN;

const LONG_FORM: u32 = 1 << 7;
const LEN_MASK: u32 = 0x7f;

/// Expand a connection list into `out`, returning how many entries it holds
/// (at most `MAX_CONN`). `fetch(i)` answers GET_CONNECT_LIST for the word
/// that starts at entry `i`; `None` ends the list where it is.
pub fn decode(len_param: u32, mut fetch: impl FnMut(u8) -> Option<u32>, out: &mut [u8; MAX_CONN]) -> u8 {
    let long = len_param & LONG_FORM != 0;
    let len = (len_param & LEN_MASK) as u8;
    let (per_word, bits) = if long { (2u8, 16u32) } else { (4u8, 8u32) };
    let range_bit = 1u32 << (bits - 1);
    let nid_mask = range_bit - 1;
    let mut n = 0usize;
    let mut prev: Option<u32> = None;
    let mut i = 0u8;
    let mut word = 0u32;
    while i < len && n < MAX_CONN {
        if i.is_multiple_of(per_word) {
            match fetch(i) {
                Some(w) => word = w,
                None => break,
            }
        }
        let e = (word >> ((i % per_word) as u32 * bits)) & ((1 << bits) - 1);
        let nid = e & nid_mask;
        if e & range_bit != 0 {
            if let Some(p) = prev {
                let mut v = p + 1;
                while v <= nid && n < MAX_CONN {
                    push(out, &mut n, v);
                    v += 1;
                }
            }
        } else {
            push(out, &mut n, nid);
        }
        prev = Some(nid);
        i += 1;
    }
    n as u8
}

/// Node ids above 127 cannot be addressed by a verb; they are kept as 0,
/// which no path will use, so the indices of the entries after them hold.
fn push(out: &mut [u8; MAX_CONN], n: &mut usize, nid: u32) {
    out[*n] = if nid > 0x7f { 0 } else { nid as u8 };
    *n += 1;
}
