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

//! The reference listing's ids, as B's marketplace_abi::ids derives them and
//! the lead checked with cast (E39). The market scope of a real listing is
//! taken under this listing_key, never under the listing's string.

use nonos_proof_abi::market_scope;
use nonos_stark::hash::keccak256;

const PUBLISHER: &str = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2";

fn unhex(s: &str) -> [u8; 32] {
    core::array::from_fn(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
}

fn key(tag: &str, parent: &[u8; 32], name: &str) -> [u8; 32] {
    let words = [keccak256(tag.as_bytes()), *parent, keccak256(name.as_bytes())];
    keccak256(&words.concat())
}

#[test]
fn qwen_small_ids_are_the_markets() {
    let listing = key("NONOS.nox.listing.v1", &unhex(PUBLISHER), "linux.qwen-small");
    let want = "ff4399cc04b31c66714c8be18d9b33e33912a83ccbc7ede50fce0a31d8414cd5";
    assert_eq!(listing, unhex(want));
    let release = key("NONOS.nox.release.v1", &listing, "qwen-small@1");
    let want = "af2e230eabc03a74fb38620601f259a086d9924a715b352c66d09c23694a864f";
    assert_eq!(release, unhex(want));
    assert_ne!(market_scope(&listing, 20_372), market_scope(&release, 20_372));
}
