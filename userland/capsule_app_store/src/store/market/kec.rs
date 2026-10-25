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
//! keccak-256 in this process, from the pinned pure sha3, as capsule_market
//! hashes for its verdicts: hashing public bytes asks the kernel for nothing,
//! so the store holds no Crypto capability.

use nonos_marketplace_abi::Keccak;
use sha3::{Digest, Keccak256};

pub struct Kec;

impl Keccak for Kec {
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        Keccak256::digest(data).into()
    }
}
