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

//! The listing and release ids a market policy carries, as
//! marketplace_abi::ids derives them; the project recomputed this vector with
//! cast. nonos-linux-pack uses marketplace_abi::ids itself at integration.

use sha3::{Digest, Keccak256};

/// The shipped index's publisher key for linux.qwen-small.
const PUBLISHER: &str = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2";

fn keccak(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = Keccak256::new();
    parts.iter().for_each(|p| h.update(p));
    h.finalize().into()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn bytes32(s: &str) -> [u8; 32] {
    core::array::from_fn(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
}

#[test]
fn qwen_smalls_listing_and_release_keys() {
    let listing_tag = keccak(&[b"NONOS.nox.listing.v1"]);
    let name = keccak(&[b"linux.qwen-small"]);
    let listing = keccak(&[&listing_tag, &bytes32(PUBLISHER), &name]);
    assert_eq!(hex(&listing), "ff4399cc04b31c66714c8be18d9b33e33912a83ccbc7ede50fce0a31d8414cd5");
    let release_tag = keccak(&[b"NONOS.nox.release.v1"]);
    let release = keccak(&[&release_tag, &listing, &keccak(&[b"qwen-small@1"])]);
    assert_eq!(hex(&release), "af2e230eabc03a74fb38620601f259a086d9924a715b352c66d09c23694a864f");
}
