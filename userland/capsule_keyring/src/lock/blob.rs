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

//! The record to and from the blob the shell keeps. Sealed under the
//! machine key wherever the TPM gives one; plain only on a machine with no
//! TPM, and a plain blob is taken back only there, so a record written in
//! the clear cannot stand in for a sealed one.

use alloc::vec::Vec;

use nonos_libc::{machine_key, MACHINE_KEY_NO_TPM};
use nonos_lock::blob::{frame, split, Kind, BLOB_MAX, SEAL_ROOM};
use nonos_lock::{Record, RECORD_LEN};
use nonos_vault::{open as vault_open, seal as vault_seal, OVERHEAD, ROOT_LABEL};

const RECORD: &[u8] = b"keyring.lock.verifier";

const _: () = assert!(OVERHEAD <= SEAL_ROOM, "the vault's framing outgrew the blob");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadError {
    /// This machine gives no key for it now: a sealed blob after the boot
    /// state changed, or a plain blob on a machine that has a TPM.
    NoKey,
    /// Not a record of this machine's.
    Malformed,
}

fn wipe(buf: &mut [u8]) {
    nonos_lock::wipe::secure_zero(buf);
}

/// The blob for `record`: sealed, or plain where there is no TPM. Empty
/// when neither can be made (a TPM that refuses this boot state, or no
/// entropy): the record then lasts this boot only.
pub fn make(record: &Record) -> Vec<u8> {
    let mut plain = record.encode();
    let mut out = [0u8; BLOB_MAX];
    let n = match machine_key(ROOT_LABEL) {
        Ok(mut root) => {
            let n = seal(&root, &plain, &mut out);
            wipe(&mut root);
            n
        }
        Err(MACHINE_KEY_NO_TPM) => frame(Kind::Plain, &plain, &mut out),
        Err(_) => None,
    };
    wipe(&mut plain);
    let blob = n.map(|n| out[..n].to_vec()).unwrap_or_default();
    wipe(&mut out);
    blob
}

fn seal(root: &[u8; 32], plain: &[u8; RECORD_LEN], out: &mut [u8; BLOB_MAX]) -> Option<usize> {
    let mut draw = [0u8; 32];
    if !crate::entropy::gather_secret(&mut draw) {
        return None;
    }
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&draw[..12]);
    wipe(&mut draw);
    let mut sealed = [0u8; RECORD_LEN + SEAL_ROOM];
    let n = vault_seal(root, RECORD, plain, &nonce, &mut sealed).ok();
    let framed = n.and_then(|n| frame(Kind::Sealed, &sealed[..n], out));
    wipe(&mut sealed);
    framed
}

/// The record in `bytes`, as `make` wrote it on this machine.
pub fn read(bytes: &[u8]) -> Result<Record, ReadError> {
    let (kind, body) = split(bytes).ok_or(ReadError::Malformed)?;
    let key = machine_key(ROOT_LABEL);
    match (kind, key) {
        (Kind::Plain, Err(MACHINE_KEY_NO_TPM)) => Record::decode(body).ok_or(ReadError::Malformed),
        (Kind::Plain, Ok(mut root)) => {
            wipe(&mut root);
            Err(ReadError::NoKey)
        }
        (Kind::Sealed, Ok(mut root)) => {
            let mut plain = [0u8; RECORD_LEN];
            let opened = vault_open(&root, RECORD, body, &mut plain);
            wipe(&mut root);
            let record = match opened {
                Ok(RECORD_LEN) => Record::decode(&plain).ok_or(ReadError::Malformed),
                _ => Err(ReadError::Malformed),
            };
            wipe(&mut plain);
            record
        }
        (_, Err(_)) => Err(ReadError::NoKey),
    }
}
