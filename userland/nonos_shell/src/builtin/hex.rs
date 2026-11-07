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

//! `hex [-d] <text...>`: the bytes of the arguments as hex, or hex back to
//! bytes with `-d`. A pure encoder; it reaches nothing outside the shell.

use alloc::vec::Vec;

use crate::flags::{parse, Spec};
use crate::out::ShellOut;

/// Encode the operands to lowercase hex, or decode hex to text with `-d`.
/// Returns false on a parse error so `hex ... && ...` gates on success.
pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    let spec = Spec::new(b"hex", b"d");
    let parsed = match parse(&spec, argv.get(1..).unwrap_or(&[])) {
        Ok(p) => p,
        Err(e) => {
            out.writeln(&e);
            return false;
        }
    };
    let input = join(&parsed.operands);
    if parsed.has(b'd') {
        match decode(&input) {
            Some(bytes) => {
                out.writeln(&bytes);
                true
            }
            None => {
                out.writeln(b"hex: not valid hex");
                false
            }
        }
    } else {
        out.writeln(&encode(&input));
        true
    }
}

/// Join the operands with a single space, matching how `echo` reads them.
fn join(operands: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, w) in operands.iter().enumerate() {
        if i > 0 {
            out.push(b' ');
        }
        out.extend_from_slice(w);
    }
    out
}

/// Lowercase hex of the bytes, shared by `rand`, `hash` and `attest`.
pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(nibble(b >> 4));
        out.push(nibble(b & 0x0f));
    }
    out
}

fn nibble(n: u8) -> u8 {
    if n < 10 {
        b'0' + n
    } else {
        b'a' + n - 10
    }
}

/// Decode hex, skipping ASCII whitespace so pasted dumps decode. An odd count
/// of digits or any non-hex byte is a failure.
fn decode(text: &[u8]) -> Option<Vec<u8>> {
    let mut digits = Vec::with_capacity(text.len());
    for &b in text {
        if b.is_ascii_whitespace() {
            continue;
        }
        digits.push(from_hex(b)?);
    }
    if digits.len() % 2 != 0 {
        return None;
    }
    Some(digits.chunks(2).map(|c| (c[0] << 4) | c[1]).collect())
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
