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

//! The HkdfLabel structure, built apart from the expansion that consumes it.

extern crate alloc;

use alloc::vec::Vec;

/*
 * RFC 8446 section 7.1. HkdfLabel: u16 length, then two single byte length
 * prefixed fields. The label carries a "tls13 " prefix inside its own count.
 */
pub fn hkdf_label(out_len: usize, label: &[u8], context: &[u8]) -> Option<Vec<u8>> {
    // Both lengths are single bytes, and the prefix costs six of the label's.
    if label.len() + 6 > 255 || context.len() > 255 || out_len > u16::MAX as usize {
        return None;
    }
    let mut info = Vec::with_capacity(label.len() + context.len() + 10);
    info.extend_from_slice(&(out_len as u16).to_be_bytes());
    info.push((label.len() + 6) as u8);
    info.extend_from_slice(b"tls13 ");
    info.extend_from_slice(label);
    info.push(context.len() as u8);
    info.extend_from_slice(context);
    Some(info)
}
