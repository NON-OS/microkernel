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

//! The serial lines nsh.attest writes about the host keys and the machine's
//! records: when the keys were made, opened or kept, and why they could
//! not be. They are where an operator learns the host key id to compare
//! with what nsh-enrol is given, and why a host serves no one. A line
//! needs Debug, which only a development twin grants; a release image
//! writes none and the client is told the refusal instead. The text is
//! pure, so nsh_front_proofs holds it.

use alloc::vec::Vec;

/// What happened to the host keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// Made now, on this machine, and sealed and kept in the store.
    Made,
    /// Made now, and not kept yet: the store keeps nothing on this boot
    /// so far. They are kept as soon as it does.
    MadeUnkept,
    /// Opened from the store, sealed by an earlier boot.
    Opened,
    /// Made earlier this boot, and now kept.
    Kept,
    /// The keys in the store were sealed under another boot state, or are
    /// not this machine's; they were set aside and new ones made.
    Replaced,
    /// The machine has no TPM: no keys, no attestation.
    NoTpm,
    /// The TPM did not give the sealing key: this errno.
    Tpm(i64),
    /// The store did not answer; asked again later.
    Store,
    /// The kernel's generator did not give the bytes.
    Entropy,
    /// The enrolment request was written for the operator.
    Request,
}

/// The line for `e`, with the host key id where one exists, newline ended.
pub fn line(e: Event, key_id: Option<&[u8; 32]>) -> Vec<u8> {
    let mut out = Vec::with_capacity(160);
    out.extend_from_slice(b"[NSH-ATTEST] ");
    out.extend_from_slice(match e {
        Event::Made => b"host keys made on this machine at first boot, sealed to the TPM and kept".as_slice(),
        Event::MadeUnkept => b"host keys made on this machine, sealed to the TPM; this boot keeps nothing yet, so they are kept once it does".as_slice(),
        Event::Opened => b"host keys opened, sealed to this TPM by an earlier boot".as_slice(),
        Event::Kept => b"host keys kept in the store".as_slice(),
        Event::Replaced => b"host keys in the store do not open in this boot state; set aside as host.keys.prev, new keys made, enrol the machine again".as_slice(),
        Event::NoTpm => b"no TPM: no host keys and no attestation; nsh serves no one".as_slice(),
        Event::Tpm(_) => b"the TPM did not give the sealing key, errno ".as_slice(),
        Event::Store => b"the store did not answer for the host keys; asked again later".as_slice(),
        Event::Entropy => b"the kernel's generator did not answer; no host keys made".as_slice(),
        Event::Request => b"enrolment request written to /nonos/nsh: host.public, enrol.nonce, enrol.doc".as_slice(),
    });
    if let Event::Tpm(rc) = e {
        if rc < 0 {
            out.push(b'-');
        }
        decimal(&mut out, rc.unsigned_abs());
    }
    if let Some(id) = key_id {
        out.extend_from_slice(b", key id ");
        hex(&mut out, id);
    }
    out.push(b'\n');
    out
}

/// Lowercase hex of `bytes`.
pub fn hex(out: &mut Vec<u8>, bytes: &[u8]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for b in bytes {
        out.push(DIGITS[usize::from(b >> 4)]);
        out.push(DIGITS[usize::from(b & 0xF)]);
    }
}

fn decimal(out: &mut Vec<u8>, mut n: u64) {
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
