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

//! The keyring's lock operations (`nonos_lock::wire`). Each request that
//! carries a passphrase is built in a buffer that is wiped when it is let
//! go; the answers carry only the count and the sealed blob.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_getpid, mk_ipc_call_timeout, mk_service_lookup};
use nonos_lock::blob::BLOB_MAX;
use nonos_lock::wire::{
    check_request, load_request, reply, set_request, state_request, Secret, COUNT_LEN,
    REPLY_HDR_LEN,
};

const SERVICE: &[u8] = b"keyring";
/// A set or a check runs Argon2id in the keyring: about 0.3 s on a laptop
/// core, far longer on a slow or emulated one.
const DERIVE_TIMEOUT_MS: u64 = 60_000;
const QUICK_TIMEOUT_MS: u64 = 1_000;
const REPLY_MAX: usize = REPLY_HDR_LEN + 1 + COUNT_LEN + BLOB_MAX;

fn port() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (rc >= 0 && port != 0).then_some(port)
}

/// The status and body of the keyring's answer, or None when it gave none.
fn call(request: Option<Secret>, seq: u32, timeout_ms: u64) -> Option<(i32, Vec<u8>)> {
    let request = request?;
    let port = port()?;
    let tx = request.as_slice();
    let mut rx = vec![0u8; REPLY_MAX];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    let n = usize::try_from(n).ok().filter(|&n| n >= REPLY_HDR_LEN)?;
    let (status, body) = reply(&rx[..n.min(rx.len())], seq)?;
    Some((status, body.to_vec()))
}

pub fn check(seq: u32, passphrase: &[u8]) -> Option<(i32, Vec<u8>)> {
    call(check_request(seq, mk_getpid(), passphrase), seq, DERIVE_TIMEOUT_MS)
}

pub fn set(seq: u32, current: &[u8], new: &[u8]) -> Option<(i32, Vec<u8>)> {
    call(set_request(seq, mk_getpid(), current, new), seq, DERIVE_TIMEOUT_MS)
}

pub fn load(seq: u32, blob: &[u8]) -> Option<(i32, Vec<u8>)> {
    call(Some(load_request(seq, mk_getpid(), blob)), seq, QUICK_TIMEOUT_MS)
}

pub fn state(seq: u32) -> Option<(i32, Vec<u8>)> {
    call(Some(state_request(seq, mk_getpid())), seq, QUICK_TIMEOUT_MS)
}
