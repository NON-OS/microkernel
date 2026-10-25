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

use nonos_nox_meter::persist::mac_key;

use super::{bound, secp};
use crate::keccak::Host;
use crate::keyring;
use crate::wipe::wipe;

/// The meter's one key and what comes from its root. No Ed25519 key and no
/// NXL1 in v1.
#[derive(Clone, Copy)]
pub struct Keys {
    /// The meter's secp256k1 key in the keyring, owned by this process.
    pub key_id: u32,
    pub address: [u8; 20],
    /// The key the state file is tagged under.
    pub mac_key: [u8; 32],
}

pub fn open() -> Result<Keys, i32> {
    let mut root = bound::root()?;
    let keys = from_root(&root);
    wipe(&mut root);
    keys
}

fn from_root(root: &[u8; 32]) -> Result<Keys, i32> {
    let key_id = secp::restore(root)?;
    let address = keyring::address(key_id)?;
    Ok(Keys { key_id, address, mac_key: mac_key(&Host, root) })
}
