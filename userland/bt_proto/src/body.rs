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

//! The bodies of the requests that carry one: FORGET, ANSWER and SCAN.

use super::ops::{PART_CONFIRM, PART_ENTER, PART_NONE};
use super::part::PASSKEY_MAX;

/// FORGET: the bond's identity address type and address, as BONDS gives
/// them.
pub fn forget_body(kind: u8, addr: &[u8; 6]) -> [u8; 7] {
    let mut b = [kind, 0, 0, 0, 0, 0, 0];
    b[1..].copy_from_slice(addr);
    b
}

/// What the user answers to PASSKEY.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply {
    /// The digits the device shows, typed here.
    Digits(u32),
    /// Whether both sides show the same number.
    Same(bool),
    /// Neither: pairing is declined.
    Decline,
}

/// ANSWER: a kind and a little-endian word. `None` for digits over six
/// places, which the capsule would refuse.
pub fn answer_body(reply: Reply) -> Option<[u8; 5]> {
    let (kind, v) = match reply {
        Reply::Digits(n) if n <= PASSKEY_MAX => (PART_ENTER, n),
        Reply::Digits(_) => return None,
        Reply::Same(same) => (PART_CONFIRM, u32::from(same)),
        Reply::Decline => (PART_NONE, 0),
    };
    let w = v.to_le_bytes();
    Some([kind, w[0], w[1], w[2], w[3]])
}

/// SCAN: 1 on, 0 off.
pub fn scan_body(on: bool) -> [u8; 1] {
    [u8::from(on)]
}
