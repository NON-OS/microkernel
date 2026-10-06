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

//! Take a key the window made back out of the keyring, with the words kept
//! beside it, when the window cannot use it.

use super::call::keyring_call;
use super::constants::OP_DELETE;

pub fn forget_key(port: u32, owner_pid: u32, id: u32) -> Result<(), i32> {
    let mut payload = [0u8; 8];
    payload[..4].copy_from_slice(&owner_pid.to_le_bytes());
    payload[4..].copy_from_slice(&id.to_le_bytes());
    keyring_call(port, OP_DELETE, &payload, 0).map(|_| ())
}
