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
//! RECEIPT and RECEIPT_ACK (letter agent-02): one boxed run's receipt, then
//! the caller's word that it is kept, and only then is the record dropped.
//!
//!   6 RECEIPT      box_len u8 | box[box_len] | policy[256]  -> NAPR v0
//!   7 RECEIPT_ACK  seal_no u64                              -> (empty)
//!
//! A job bound to RECEIPT answers its caller, and no one else, with the
//! receipt; an ack for any other seal number is refused and drops nothing.

pub const RECEIPT: u16 = 6;
pub const RECEIPT_ACK: u16 = 7;

const POLICY: usize = 256;

pub struct ReceiptRequest<'a> {
    pub box_word: &'a [u8],
    pub policy: &'a [u8; POLICY],
}

impl<'a> ReceiptRequest<'a> {
    pub fn encode(&self) -> Option<alloc::vec::Vec<u8>> {
        let len = u8::try_from(self.box_word.len()).ok().filter(|&n| n > 0)?;
        Some([&[len][..], self.box_word, &self.policy[..]].concat())
    }

    /// One box word, 1 to 255 bytes, and one policy, and nothing more.
    pub fn decode(b: &'a [u8]) -> Option<ReceiptRequest<'a>> {
        let (&len, rest) = b.split_first()?;
        let len = usize::from(len);
        if len == 0 || rest.len() != len + POLICY {
            return None;
        }
        Some(ReceiptRequest { box_word: &rest[..len], policy: rest[len..].try_into().ok()? })
    }
}

/// The seal number an ack names, exactly eight bytes.
pub fn ack_seal_no(b: &[u8]) -> Option<u64> {
    Some(u64::from_le_bytes(b.try_into().ok()?))
}
