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

//! The record as it is kept on disk: one kind byte, then the body. Sealed:
//! a `nonos_vault` blob of the record under the machine key, which the
//! keyring opens only on this machine in this boot state. Plain: the record
//! itself, which the keyring takes only on a machine with no TPM to seal
//! under.

use crate::record::RECORD_LEN;

pub const KIND_SEALED: u8 = 1;
pub const KIND_PLAIN: u8 = 2;

/// Room the vault's framing may take around a sealed record.
pub const SEAL_ROOM: usize = 128;

/// The largest blob either kind makes.
pub const BLOB_MAX: usize = 1 + RECORD_LEN + SEAL_ROOM;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Sealed,
    Plain,
}

/// The kind and body of `bytes`, or None when it is neither.
pub fn split(bytes: &[u8]) -> Option<(Kind, &[u8])> {
    let (&kind, body) = bytes.split_first()?;
    if bytes.len() > BLOB_MAX {
        return None;
    }
    match kind {
        KIND_SEALED if !body.is_empty() => Some((Kind::Sealed, body)),
        KIND_PLAIN if body.len() == RECORD_LEN => Some((Kind::Plain, body)),
        _ => None,
    }
}

/// `body` framed as `kind` into `out`; the bytes written, or None when it
/// does not fit.
pub fn frame(kind: Kind, body: &[u8], out: &mut [u8]) -> Option<usize> {
    let n = 1 + body.len();
    if n > BLOB_MAX || n > out.len() {
        return None;
    }
    out[0] = match kind {
        Kind::Sealed => KIND_SEALED,
        Kind::Plain => KIND_PLAIN,
    };
    out[1..n].copy_from_slice(body);
    Some(n)
}
