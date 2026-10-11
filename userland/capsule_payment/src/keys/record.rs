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

//! Records sealed under a key the bound root derives for each record name
//! alone (nonos_vault), with a fresh nonce from the kernel every time.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::crypto_random;
use nonos_payment_proto::{EAGAIN, EIO};
use nonos_seal::NONCE_LEN;
use nonos_vault::OVERHEAD;

use super::bound;
use crate::wipe::wipe;

/// `out` filled from the kernel's generator; never all zero.
pub fn random(out: &mut [u8]) -> Result<(), i32> {
    let short = crypto_random(out.as_mut_ptr(), out.len()) < out.len() as i64;
    if short || out.iter().all(|b| *b == 0) {
        return Err(EAGAIN);
    }
    Ok(())
}

pub fn seal_record(record: &[u8], plain: &[u8]) -> Result<Vec<u8>, i32> {
    let mut nonce = [0u8; NONCE_LEN];
    random(&mut nonce)?;
    let mut root = bound::root()?;
    let mut out = vec![0u8; plain.len() + OVERHEAD];
    let sealed = nonos_vault::seal(&root, record, plain, &nonce, &mut out);
    wipe(&mut root);
    sealed.map_err(|_| EAGAIN)?;
    Ok(out)
}

/// The record's bytes, or EIO for one that does not open under this root.
pub fn open_record(record: &[u8], blob: &[u8]) -> Result<Vec<u8>, i32> {
    let mut root = bound::root()?;
    let mut out = vec![0u8; blob.len().saturating_sub(OVERHEAD)];
    let opened = nonos_vault::open(&root, record, blob, &mut out);
    wipe(&mut root);
    if opened.is_err() {
        // Whatever an open that failed wrote is not left behind.
        wipe(&mut out);
        return Err(EIO);
    }
    Ok(out)
}
