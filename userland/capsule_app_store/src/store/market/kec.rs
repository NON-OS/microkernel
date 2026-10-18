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
//! keccak-256 through the crypto service, as the wallet hashes a payee's
//! checksum, behind marketplace_abi's trait.

use nonos_marketplace_abi::Keccak;

pub struct Kec;

impl Keccak for Kec {
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        if nonos_libc::crypto_keccak256(data.as_ptr(), data.len(), out.as_mut_ptr(), 32) != 32 {
            // As the wallet's: a hash the service did not give is no hash.
            out = [0u8; 32];
        }
        out
    }
}
