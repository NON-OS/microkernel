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

//! The two answers that need AttestRead: the kernel's NONOSATT v5 document
//! over the challenge nsh.front relays, and the attestation registry. Each
//! is the kernel's own bytes, unaltered; this capsule neither signs nor
//! shapes them, so the quote the front presents is the machine's, bound to
//! the handshake's challenge.

use crate::reply::{err, frame};
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_attest_doc, mk_attest_entries, ATTEST_ENTRY_LEN};
use nsh_front_wire::{CHALLENGE_LEN, OP_DOC, OP_ENTRIES, ST_ATTEST_REFUSED, ST_BAD_REQUEST, ST_OK};

/// A NONOSATT v5 document, with room to spare under the kernel's 1 MiB IPC
/// message limit once this capsule's reply header is added, so the answer
/// always fits one relay to nsh.front. A development, path-only image carries
/// a small firmware event log and stays far under this; a full event log near
/// 1 MiB would need chunking the relay, which this wire does not yet do. The
/// kernel refuses an overflow without saying how much it needed, so the length
/// is read back rather than probed.
const DOC_CAP: usize = (1 << 20) - 4096;
/// 4096 registry entries: more capsules than any image runs at once.
const ENTRIES_CAP: usize = 4096 * ATTEST_ENTRY_LEN;

/// The document over `body`, which must be exactly the 32-byte challenge.
pub fn document(body: &[u8]) -> Vec<u8> {
    let challenge: [u8; CHALLENGE_LEN] = match body.try_into() {
        Ok(c) => c,
        Err(_) => return err(OP_DOC, ST_BAD_REQUEST),
    };
    match quote(&challenge) {
        Some(doc) => frame(OP_DOC, ST_OK, &doc),
        None => err(OP_DOC, ST_ATTEST_REFUSED),
    }
}

/// The kernel's document bound to `challenge`, or `None` when it refused.
pub fn quote(challenge: &[u8; CHALLENGE_LEN]) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; DOC_CAP];
    let rc = mk_attest_doc(challenge, &mut buf);
    let len = usize::try_from(rc).ok().filter(|&n| n <= buf.len())?;
    buf.truncate(len);
    Some(buf)
}

/// The attestation registry's entries, as the kernel folds them.
pub fn entries() -> Vec<u8> {
    let mut buf = vec![0u8; ENTRIES_CAP];
    let rc = mk_attest_entries(&mut buf);
    if rc < 0 {
        return err(OP_ENTRIES, ST_ATTEST_REFUSED);
    }
    buf.truncate(rc as usize);
    frame(OP_ENTRIES, ST_OK, &buf)
}
