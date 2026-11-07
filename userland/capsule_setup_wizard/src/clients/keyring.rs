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

//! OP_LOCK_SET to the keyring: the first passphrase, with no current one.
//! The request is built in a buffer wiped when it is let go.

use nonos_libc::{mk_getpid, mk_ipc_call_timeout, mk_service_lookup};
use nonos_lock::wire::{answer, reply, set_request, Answer, REPLY_HDR_LEN};

const SERVICE: &[u8] = b"keyring";
/// The keyring derives Argon2id before it answers.
const TIMEOUT_MS: u64 = 60_000;
const SEQ: u32 = 1;

/// Whether the keyring took `passphrase` as the lock screen's.
pub fn set(passphrase: &[u8]) -> bool {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return false;
    }
    let Some(request) = set_request(SEQ, mk_getpid(), b"", passphrase) else {
        return false;
    };
    let tx = request.as_slice();
    let mut rx = [0u8; REPLY_HDR_LEN + nonos_lock::blob::BLOB_MAX];
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len(), TIMEOUT_MS);
    let Ok(n) = usize::try_from(n) else { return false };
    matches!(
        reply(&rx[..n.min(rx.len())], SEQ).map(|(s, b)| answer(s, b)),
        Some(Answer::Done(_))
    )
}
