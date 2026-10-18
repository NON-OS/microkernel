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

//! One NDP of a received block (USB NCM 1.0, 3.3, as recalled,
//! unverified): dwSignature, wLength, the next NDP's index, then
//! (index, length) entries, the last of them the zero pair that ends the
//! list. The whole NDP must lie in the block. Its datagrams are taken in
//! order up to the first zero or malformed entry, each wholly inside the
//! block and at least an Ethernet header long.

use alloc::collections::VecDeque;

use nonos_usbnet::nic::ETH_HEADER;

use super::format::Format;
use super::nth::{le16, le32};

/// The next NDP's index (0: none), or `None` for an NDP that does not fit
/// in `b`. At most `room` datagrams are left in `out`.
pub fn ndp(
    f: &Format,
    b: &[u8],
    at: usize,
    room: usize,
    out: &mut VecDeque<(usize, usize)>,
) -> Option<usize> {
    let len = le16(b, at + 4)?;
    let entry = f.entry;
    if len < f.ndp_header + 2 * entry || at.checked_add(len)? > b.len() {
        return None;
    }
    let next = f.field(b, at + f.ndp_next)?;
    if le32(b, at)? != f.ndp_sign as usize {
        return Some(next);
    }
    // The last slot wLength makes room for is the terminator.
    let slots = (len - f.ndp_header) / entry - 1;
    for x in 0..slots {
        let e = at + f.ndp_header + x * entry;
        let (index, dlen) = (f.field(b, e)?, f.field(b, e + entry / 2)?);
        let inside = index.checked_add(dlen).is_some_and(|end| end <= b.len());
        if out.len() >= room || index == 0 || dlen < ETH_HEADER || !inside {
            break;
        }
        out.push_back((index, dlen));
    }
    Some(next)
}
