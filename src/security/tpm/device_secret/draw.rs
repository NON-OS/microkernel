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

//! The secret drawn from the key: HMAC blocks under the policy session, each
//! cut into eight-byte words, a word kept only below p.

use alloc::vec::Vec;

use crate::security::hardening::memory_sanitization::secure_zero_slice;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{TPM_ALG_SHA256, TPM_CC_HMAC, TPM_ST_SESSIONS};
use crate::security::tpm::machine_key::hmac::parse_hmac;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::wire::frame;
use crate::security::tpm::machine_key::KeyError;

/// The Goldilocks modulus: a word is kept only below it, so no lane is biased.
const P: u64 = 0xFFFF_FFFF_0000_0001;

const LABEL: &[u8] = b"NONOS-DEVICE-SECRET-v1";

/// HMAC blocks tried; each word is rejected with probability about 2^-32.
const MAX_BLOCKS: u8 = 4;

/// HMAC under the policy session, kept open between blocks.
fn build_hmac_continued(key: u32, session: u32, block: u8) -> Vec<u8> {
    let mut body = Vec::with_capacity(24 + LABEL.len() + 1);
    body.extend_from_slice(&key.to_be_bytes());
    // Authorisation: the policy session, no nonce, continueSession set, empty HMAC.
    let mut auth = [0u8; 9];
    auth[..4].copy_from_slice(&session.to_be_bytes());
    auth[6] = 0x01;
    body.extend_from_slice(&(auth.len() as u32).to_be_bytes());
    body.extend_from_slice(&auth);
    body.extend_from_slice(&((LABEL.len() + 1) as u16).to_be_bytes());
    body.extend_from_slice(LABEL);
    body.push(block);
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    frame(TPM_ST_SESSIONS, TPM_CC_HMAC, &body)
}

pub(super) fn draw(key: u32, session: u32) -> Result<[u64; 4], KeyError> {
    let mut out = [0u64; 4];
    let mut filled = 0usize;
    for block in 0..MAX_BLOCKS {
        let mut bytes = parse_hmac(&run(&build_hmac_continued(key, session, block))?)?;
        for chunk in bytes.chunks_exact(8) {
            let mut w = [0u8; 8];
            w.copy_from_slice(chunk);
            let v = u64::from_le_bytes(w);
            if v < P && filled < 4 {
                out[filled] = v;
                filled += 1;
            }
        }
        secure_zero_slice(&mut bytes);
        if filled == 4 {
            return Ok(out);
        }
    }
    Err(TpmError::InvalidResponse.into())
}
