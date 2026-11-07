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

//! `b64 [-d] <text...>`: base64 of the arguments, or base64 back to text with
//! `-d`. The encoder is `nonos_base64`, the same one the shell already uses to
//! carry credentials; it reaches nothing outside the shell.

use alloc::vec::Vec;

use crate::flags::{parse, Spec};
use crate::out::ShellOut;

/// Encode the operands to base64, or decode base64 to text with `-d`. Returns
/// false on a parse or decode error so the status reflects it.
pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    let spec = Spec::new(b"b64", b"d");
    let parsed = match parse(&spec, argv.get(1..).unwrap_or(&[])) {
        Ok(p) => p,
        Err(e) => {
            out.writeln(&e);
            return false;
        }
    };
    let input = join(&parsed.operands);
    if parsed.has(b'd') {
        let Ok(text) = core::str::from_utf8(&input) else {
            out.writeln(b"b64: input is not valid text");
            return false;
        };
        match nonos_base64::decode_b64(text) {
            Some(bytes) => {
                out.writeln(&bytes);
                true
            }
            None => {
                out.writeln(b"b64: not valid base64");
                false
            }
        }
    } else {
        out.writeln(&nonos_base64::encode(&input));
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
