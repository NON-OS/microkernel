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

//! `hash [-a sha256|sha512] <text...>`: the digest of the arguments.
//!
//! The digest is computed in the shell by `nonos_hash`, the same pure
//! implementation the rest of userland links; it reaches no syscall and leaves
//! no state. The argument words are joined with one space, as `echo` reads
//! them, and the lowercase hex of the digest is written on one line.

use alloc::vec::Vec;

use crate::flags::{parse, Spec};
use crate::out::ShellOut;

/// Hash the operands and print the digest. Returns false on a bad flag or an
/// unknown algorithm, so the status reflects it.
pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    let spec = Spec::new(b"hash", b"").valued(b"a");
    let parsed = match parse(&spec, argv.get(1..).unwrap_or(&[])) {
        Ok(p) => p,
        Err(e) => {
            out.writeln(&e);
            return false;
        }
    };
    let algo = parsed.value(b'a').unwrap_or(b"sha256");
    let input = join(&parsed.operands);
    let digest = match algo {
        b"sha256" | b"256" => nonos_hash::sha256(&input).to_vec(),
        b"sha512" | b"512" => nonos_hash::sha512(&input).to_vec(),
        _ => {
            out.writeln(b"hash: algorithm is sha256 or sha512");
            return false;
        }
    };
    out.writeln(&super::hex::encode(&digest));
    true
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
