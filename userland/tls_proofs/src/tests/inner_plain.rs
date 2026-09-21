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

//! Finding the true content type under a record's padding.

use crate::inner_plain::split;

/*
 * RFC 8446 section 5.2. Inside a protected record the plaintext is the content,
 * then the true content type, then any number of zero bytes of padding. The type
 * is therefore the last non-zero byte, and everything before it is content.
 *
 * Getting this wrong decides whether a record is read as handshake or as
 * application data, which is what tells the handshake where it ended.
 */
#[test]
fn the_type_is_the_last_byte_that_is_not_padding() {
    let (content, kind) = split(&[1, 2, 3, 22]).expect("a record with no padding");
    assert_eq!(content, &[1, 2, 3]);
    assert_eq!(kind, 22, "handshake");
}
#[test]
fn padding_is_stripped_whatever_its_length() {
    for pad in 0..40usize {
        let mut record = vec![0xAA, 0xBB];
        record.push(23); // application_data
        record.extend(core::iter::repeat_n(0u8, pad));
        let (content, kind) = split(&record).unwrap_or_else(|| panic!("{pad} bytes of padding"));
        assert_eq!(content, &[0xAA, 0xBB]);
        assert_eq!(kind, 23);
    }
}
