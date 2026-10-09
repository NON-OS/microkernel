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
 * A further account of this wallet's phrase, account `index` of
 * m/44'/60'/0'/0/i, made by the keyring from the words it keeps beside the
 * root account. The words never reach the wallet for this.
 */

use super::call::keyring_call;
use super::constants::{HDR_LEN, OP_WALLET_DERIVE};

pub fn derive_account(
    port: u32,
    owner_pid: u32,
    root: u32,
    index: u32,
    now: u64,
    expires_at: u64,
) -> Result<u32, i32> {
    let mut payload = [0u8; 28];
    payload[..4].copy_from_slice(&owner_pid.to_le_bytes());
    payload[4..8].copy_from_slice(&root.to_le_bytes());
    payload[8..12].copy_from_slice(&index.to_le_bytes());
    payload[12..20].copy_from_slice(&now.to_le_bytes());
    payload[20..].copy_from_slice(&expires_at.to_le_bytes());
    let rx = keyring_call(port, OP_WALLET_DERIVE, &payload, 4)?;
    let body = rx.get(HDR_LEN..HDR_LEN + 4).ok_or(-11)?;
    Ok(u32::from_le_bytes([body[0], body[1], body[2], body[3]]))
}
