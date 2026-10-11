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
//! The state file with its tag, so a file the meter did not write is refused.
//! The vfs lets any FileSystem holder unlink and recreate a name, and a record
//! restored at boot has no owner, so the bytes alone prove nothing:
//!
//!   file = body (STATE_LEN) | tag[32]
//!   tag  = keccak256("NONOS.nox.meter.state.v1" | mac_key | body)
//!
//! mac_key = keccak256("NONOS.nox.meter.mac.v1" | root), root being the machine
//! key under the meter's caller-bound label, which no other capsule derives.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use super::decode::decode;
use super::encode::encode;
use super::layout::STATE_LEN;
use crate::error::MeterError;
use crate::state::MeterState;

pub const FILE_LEN: usize = STATE_LEN + 32;
const STATE_TAG: &[u8] = b"NONOS.nox.meter.state.v1";
const MAC_TAG: &[u8] = b"NONOS.nox.meter.mac.v1";

pub fn mac_key(k: &impl Keccak, root: &[u8; 32]) -> [u8; 32] {
    let mut pre = Vec::with_capacity(MAC_TAG.len() + 32);
    pre.extend_from_slice(MAC_TAG);
    pre.extend_from_slice(root);
    k.keccak256(&pre)
}

fn tag(k: &impl Keccak, key: &[u8; 32], body: &[u8]) -> [u8; 32] {
    let mut pre = Vec::with_capacity(STATE_TAG.len() + 32 + body.len());
    pre.extend_from_slice(STATE_TAG);
    pre.extend_from_slice(key);
    pre.extend_from_slice(body);
    k.keccak256(&pre)
}

/// The state as the file the meter writes, FILE_LEN bytes.
pub fn seal(k: &impl Keccak, key: &[u8; 32], s: &MeterState) -> Option<Vec<u8>> {
    let mut out = encode(s)?;
    let t = tag(k, key, &out);
    out.extend_from_slice(&t);
    Some(out)
}

/// The state, if `file` is one the holder of `key` wrote.
pub fn open(k: &impl Keccak, key: &[u8; 32], file: &[u8]) -> Result<MeterState, MeterError> {
    if file.len() != FILE_LEN {
        return Err(MeterError::Corrupt);
    }
    let (body, got) = file.split_at(STATE_LEN);
    let want = tag(k, key, body);
    let diff = want.iter().zip(got).fold(0u8, |d, (a, b)| d | (a ^ b));
    if diff != 0 {
        return Err(MeterError::Corrupt);
    }
    decode(body)
}
