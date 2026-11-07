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

//! The serial line nsh.front writes when a relay to nsh.attest fails. The
//! client is told only BUSY, as nsh/1 says it must be, so this line is where
//! an operator learns which call failed, how, and after how long. Pure, so
//! nsh_front_proofs holds it.

use alloc::vec::Vec;

/// nsh.attest does not take calls from this process (nsh_front_wire).
pub const ST_REFUSED_CALLER: u16 = 2;
/// The kernel would not produce the document (nsh_front_wire).
pub const ST_ATTEST_REFUSED: u16 = 3;
/// The machine has no TPM (nsh_front_wire).
pub const ST_NO_TPM: u16 = 5;
/// The host keys could not be made or opened now (nsh_front_wire).
pub const ST_NO_KEYS: u16 = 6;

/// How one relay failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The call itself failed: the kernel's errno, a timeout among them.
    NoAnswer(i64),
    /// nsh.attest answered with this status.
    Status(u16),
    /// The answer was not a whole nsh.front frame.
    Unframed,
}

/// The name a relayed op is said by.
pub fn op_name(op: u16) -> &'static str {
    match op {
        1 => "doc",
        2 => "entries",
        3 => "keys",
        4 => "sign",
        5 => "records",
        _ => "health",
    }
}

/// The line for a failed relay of `op` (`doc`, `entries`, `keys`, `sign`
/// or `records`) that took
/// `waited_ms`, newline ended.
pub fn line(op: &str, fault: Fault, waited_ms: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(128);
    out.extend_from_slice(b"[NSH-FRONT] attest relay ");
    out.extend_from_slice(op.as_bytes());
    out.extend_from_slice(b": ");
    match fault {
        Fault::NoAnswer(errno) => {
            out.extend_from_slice(b"no answer from nsh.attest, kernel errno ");
            signed(&mut out, errno);
        }
        Fault::Status(ST_REFUSED_CALLER) => {
            out.extend_from_slice(b"nsh.attest refused this caller")
        }
        Fault::Status(ST_ATTEST_REFUSED) => out.extend_from_slice(b"the kernel would not attest"),
        Fault::Status(ST_NO_TPM) => {
            out.extend_from_slice(b"the machine has no TPM, so it has no host keys")
        }
        Fault::Status(ST_NO_KEYS) => {
            out.extend_from_slice(b"nsh.attest could not make or open the host keys now")
        }
        Fault::Status(st) => {
            out.extend_from_slice(b"nsh.attest answered status ");
            unsigned(&mut out, u64::from(st));
        }
        Fault::Unframed => out.extend_from_slice(b"the answer was not a whole frame"),
    }
    out.extend_from_slice(b", after ");
    unsigned(&mut out, waited_ms);
    out.extend_from_slice(b" ms\n");
    out
}

fn signed(out: &mut Vec<u8>, n: i64) {
    if n < 0 {
        out.push(b'-');
    }
    unsigned(out, n.unsigned_abs());
}

fn unsigned(out: &mut Vec<u8>, mut n: u64) {
    let mut digits = [0u8; 20];
    let mut at = digits.len();
    loop {
        at -= 1;
        digits[at] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    out.extend_from_slice(&digits[at..]);
}
