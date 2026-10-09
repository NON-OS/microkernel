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

//! What every command here marshals the same way, and the one step every
//! parser here repeats.

use alloc::vec::Vec;

use super::consts::TPM_RS_PW;
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;

/// A TPM2B: the 16-bit size, then the bytes. Every caller bounds the bytes
/// far below 64 KiB first.
pub(super) fn put_tpm2b(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
    out.extend_from_slice(bytes);
}

/// One authorization: the session, no nonce, no attributes, and an empty
/// HMAC or password.
pub(super) fn auth(session: u32) -> [u8; 9] {
    let mut a = [0u8; 9];
    a[..4].copy_from_slice(&session.to_be_bytes());
    a
}

/// An authorization area of one password session with the empty password.
pub(super) fn put_password(out: &mut Vec<u8>) {
    out.extend_from_slice(&9u32.to_be_bytes());
    out.extend_from_slice(&auth(TPM_RS_PW));
}

/// Steps over a TPM2B in a response, refusing one that runs past its end.
pub(super) fn skip_tpm2b(c: &mut Cursor) -> Result<(), TpmError> {
    let n = c.u16()? as usize;
    c.skip(n)
}
