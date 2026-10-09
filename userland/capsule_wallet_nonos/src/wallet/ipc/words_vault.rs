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

/*
 * The recovery words sealed to this machine by the keyring, kept beside the
 * account vault, so further accounts and the shield still open from the
 * phrase after a reboot. The wallet holds only the sealed blob.
 */

use nonos_seal::{NONCE_LEN, TAG_LEN};

use super::call::keyring_call;
use super::constants::{HDR_LEN, OP_SHIELD_OPEN, OP_SHIELD_SEAL};

/* The sealed record: vault header 12, nonce, count 1 and 24 indices, tag. */
pub const WORDS_BLOB_LEN: usize = 12 + NONCE_LEN + 1 + 2 * 24 + TAG_LEN;

pub fn seal_words(port: u32, owner_pid: u32, wallet_id: u32) -> Result<[u8; WORDS_BLOB_LEN], i32> {
    let mut payload = [0u8; 8];
    payload[..4].copy_from_slice(&owner_pid.to_le_bytes());
    payload[4..].copy_from_slice(&wallet_id.to_le_bytes());
    let rx = keyring_call(port, OP_SHIELD_SEAL, &payload, WORDS_BLOB_LEN)?;
    let body = rx.get(HDR_LEN..HDR_LEN + WORDS_BLOB_LEN).ok_or(-11)?;
    let mut out = [0u8; WORDS_BLOB_LEN];
    out.copy_from_slice(body);
    Ok(out)
}

/* Open sealed words beside the account `wallet_id` restored first. */
pub fn open_words(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    now: u64,
    expires_at: u64,
    blob: &[u8; WORDS_BLOB_LEN],
) -> Result<(), i32> {
    let mut payload = alloc::vec::Vec::with_capacity(24 + WORDS_BLOB_LEN);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    payload.extend_from_slice(&now.to_le_bytes());
    payload.extend_from_slice(&expires_at.to_le_bytes());
    payload.extend_from_slice(blob);
    keyring_call(port, OP_SHIELD_OPEN, &payload, 0).map(|_| ())
}
