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

//! Sealing the host keys to this machine's TPM and opening them again. The
//! root is the TPM's machine key under nsh.attest's bound label: the same
//! 32 bytes on every boot of this machine in this boot state, and no
//! other machine's. It is fetched, used and wiped within one call. The
//! sealed bytes are a vault blob (nonos_vault), authenticated, so a file
//! anyone else wrote does not open.

use nonos_libc::{machine_key, MACHINE_KEY_NO_TPM};
use nonos_vault::{open as vault_open, seal as vault_seal, OVERHEAD};

use super::label::{root_label, RECORD};
use super::layout::PLAIN_LEN;
use super::Fault;
use crate::rng;

/// The sealed file's one length.
pub const SEALED_LEN: usize = PLAIN_LEN + OVERHEAD;

fn root() -> Result<[u8; 32], Fault> {
    machine_key(&root_label()).map_err(|rc| {
        if rc == MACHINE_KEY_NO_TPM {
            Fault::NoTpm
        } else {
            Fault::Tpm(rc)
        }
    })
}

/// Whether this machine gives the root now.
pub fn probe() -> Result<(), Fault> {
    let mut key = root()?;
    wipe(&mut key);
    Ok(())
}

/// `plain` sealed, or why it could not be.
pub fn seal(plain: &[u8; PLAIN_LEN], out: &mut [u8; SEALED_LEN]) -> Result<(), Fault> {
    let mut nonce = [0u8; 12];
    if !rng::fill(&mut nonce) {
        return Err(Fault::Entropy);
    }
    let mut key = root()?;
    let sealed = vault_seal(&key, RECORD, plain, &nonce, out);
    wipe(&mut key);
    match sealed {
        Ok(SEALED_LEN) => Ok(()),
        _ => {
            wipe(out);
            Err(Fault::Entropy)
        }
    }
}

/// The plaintext of `sealed`, written into `plain`.
pub fn open(sealed: &[u8], plain: &mut [u8; PLAIN_LEN]) -> Result<(), Fault> {
    if sealed.len() != SEALED_LEN {
        return Err(Fault::Foreign);
    }
    let mut key = root()?;
    let opened = vault_open(&key, RECORD, sealed, plain);
    wipe(&mut key);
    match opened {
        Ok(PLAIN_LEN) => Ok(()),
        _ => {
            wipe(plain);
            Err(Fault::Foreign)
        }
    }
}

/// Zero `buf` in a way the optimiser keeps.
pub fn wipe(buf: &mut [u8]) {
    nonos_hash::wipe(buf);
}
