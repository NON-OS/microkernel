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

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_marketplace_abi::{encode_and_sign, MarketplaceIndex};

use crate::release_tests::index;

pub(super) const FUZZ_ROUNDS: usize = 200_000;

pub(super) fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn blob_of(index: MarketplaceIndex) -> Vec<u8> {
    encode_and_sign(index, |_| [7; 64]).expect("a well formed index encodes").0
}

/// Indexes of zero, one and three listings, encoded as the operator would.
pub(super) fn seeds() -> Vec<Vec<u8>> {
    let mut empty = index();
    empty.entries.clear();
    let mut three = index();
    let mut second = three.entries[0].clone();
    second.listing_id = String::from("linux.yq");
    second.releases[0].supported_arches.push(String::from("aarch64-linux"));
    second.releases[0].required_capabilities = vec![String::from("net"), String::from("fs")];
    let mut third = second.clone();
    third.listing_id = String::from("linux.zq");
    third.releases.clear();
    three.entries.push(second);
    three.entries.push(third);
    vec![blob_of(empty), blob_of(index()), blob_of(three)]
}
