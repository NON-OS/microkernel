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
//! The keyring's tx_fields, named fields in, against RLP written here by
//! hand: every field in its EIP-1559 place, so no two can be swapped.

use crate::server::eip1559::tx_fields::{tx_fields, TxFields};

/// A big-endian integer as RLP: leading zeros dropped, zero as 0x80.
fn uint(be: &[u8]) -> Vec<u8> {
    let v: Vec<u8> = be.iter().copied().skip_while(|b| *b == 0).collect();
    if v.len() == 1 && v[0] < 0x80 {
        return v;
    }
    string(&v)
}

/// A short byte string as RLP (all of these are under 56 bytes).
fn string(b: &[u8]) -> Vec<u8> {
    let mut out = vec![0x80 + b.len() as u8];
    out.extend_from_slice(b);
    out
}

fn word(last: &[u8]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[32 - last.len()..].copy_from_slice(last);
    w
}

#[test]
fn each_field_lands_in_its_eip1559_place() {
    let (nonce, tip, cap, gas, value) = (
        word(&[7]),
        word(&[0x3b, 0x9a]),
        word(&[0x77, 0x35, 0x94, 0x00]),
        word(&[0x52, 0x08]),
        word(&[0]),
    );
    let to = [0xab; 20];
    let data = [0xa9, 0x05, 0x9c, 0xbb];
    let t = TxFields {
        chain_id: &[0xaa, 0x36, 0xa7],
        nonce: &nonce,
        max_priority: &tip,
        max_fee: &cap,
        gas: &gas,
        to: &to,
        value: &value,
        data: &data,
    };
    let want = vec![
        uint(&[0xaa, 0x36, 0xa7]),
        uint(&nonce),
        uint(&tip),
        uint(&cap),
        uint(&gas),
        string(&to),
        uint(&value),
        string(&data),
        vec![0xc0],
    ];
    assert_eq!(want[1], vec![0x07], "a nonce under 0x80 is its own byte");
    assert_eq!(want[6], vec![0x80], "zero is the empty string");
    assert_eq!(tx_fields(&t), want);
}
