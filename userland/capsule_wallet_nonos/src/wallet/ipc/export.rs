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

use alloc::vec::Vec;

use super::call::{keyring_call, wipe};
use super::constants::{HDR_LEN, OP_WALLET_EXPORT};

// Ask the keyring for this wallet's raw private key, written straight into
// the caller's `out`. The keyring returns it only to the owning process. The
// caller must wipe `out` the moment it is done with it; nothing here keeps a
// copy, not even one returned by value.
pub fn export_secret(
    port: u32,
    owner_pid: u32,
    wallet_id: u32,
    out: &mut [u8; 32],
) -> Result<(), i32> {
    let mut payload = Vec::with_capacity(8);
    payload.extend_from_slice(&owner_pid.to_le_bytes());
    payload.extend_from_slice(&wallet_id.to_le_bytes());
    let mut rx = keyring_call(port, OP_WALLET_EXPORT, &payload, 32)?;
    if rx.len() < HDR_LEN + 32 {
        wipe(&mut rx);
        return Err(-11);
    }
    out.copy_from_slice(&rx[HDR_LEN..HDR_LEN + 32]);
    wipe(&mut rx);
    Ok(())
}
