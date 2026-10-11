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

//! The escrow install pin word (v1.4.25): 145 lowercase characters, read
//! back exactly, and nothing else read at all.

use nonos_nox_chunks::{EscrowPin, PIN_LEN};

fn pin() -> EscrowPin {
    EscrowPin { chunk_root: [0xab; 32], content_len: 0x0102_0304_0506_0708, order_id: [0x0f; 32] }
}

#[test]
fn the_word_is_e_then_three_lowercase_hex_fields() {
    let w = pin().word();
    assert_eq!(PIN_LEN, 145);
    assert_eq!(w[0], b'e');
    assert_eq!(&w[1..65], "ab".repeat(32).as_bytes());
    assert_eq!(&w[65..81], b"0102030405060708", "content_len big endian");
    assert_eq!(&w[81..], "0f".repeat(32).as_bytes());
    assert_eq!(EscrowPin::parse(&w), Some(pin()));
}

#[test]
fn nothing_else_parses() {
    let w = pin().word();
    let no = |b: &[u8], why: &str| assert_eq!(EscrowPin::parse(b), None, "{why}");
    no(&w[..PIN_LEN - 1], "short");
    no(&[w.as_slice(), b"0"].concat(), "long");
    let mut upper = w;
    upper[1] = b'A';
    no(&upper, "upper case");
    let mut lead = w;
    lead[0] = b'E';
    no(&lead, "another lead");
    let mut bad = w;
    bad[100] = b'g';
    no(&bad, "not hex");
    no(&[0u8; 32], "a BLAKE3 pin is not this word");
}
