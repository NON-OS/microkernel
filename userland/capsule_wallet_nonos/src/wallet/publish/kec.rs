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
 * keccak-256 through the crypto service, as the wallet hashes everything
 * else on the device, behind marketplace_abi's trait.
 */

use core::cell::Cell;

use nonos_marketplace_abi::Keccak;

pub struct Kec;

impl Keccak for Kec {
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        if nonos_libc::crypto_keccak256(data.as_ptr(), data.len(), out.as_mut_ptr(), 32) != 32 {
            // A hash the service did not give is no hash: an all-zero word
            // matches no id, typehash or digest the screen checks against.
            out = [0u8; 32];
        }
        out
    }
}

/// The crypto service's keccak, noting whether it ever failed to answer,
/// so a checksum it could not check is said to be unchecked, not wrong.
#[derive(Default)]
pub struct Noted {
    missed: Cell<bool>,
}

impl Noted {
    pub fn answered(&self) -> bool {
        !self.missed.get()
    }
}

impl Keccak for Noted {
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        self.missed.set(self.missed.get() | !crate::wallet::tx_hash::tx_hash(data, &mut out));
        out
    }
}
