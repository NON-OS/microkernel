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

//! The ids of the reference listing (plan/25), from the shipped catalogue's
//! own publisher key. A and C pin the same two words; a change here is a
//! change to every policy and proof that names them.

use nonos_marketplace_abi::{decode_index, listing_id, release_id};

use super::hosts::Host;

static BASELINE: &[u8] = include_bytes!("../../../../nonos-data/market/index.bin");

const KEY: &str = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2";
const LISTING_KEY: &str = "ff4399cc04b31c66714c8be18d9b33e33912a83ccbc7ede50fce0a31d8414cd5";
const RELEASE_KEY: &str = "af2e230eabc03a74fb38620601f259a086d9924a715b352c66d09c23694a864f";

fn hex(b: &[u8; 32]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn linux_qwen_small_at_qwen_small_1_is_pinned() {
    let d = decode_index(BASELINE).expect("the shipped catalogue decodes");
    let e = d.index.entries.iter().find(|e| e.listing_id == "linux.qwen-small").expect("listed");
    assert_eq!(hex(&e.publisher_pubkey), KEY);
    assert!(e.releases.iter().any(|r| r.release_id == "qwen-small@1"));
    let listing = listing_id(&Host, &e.publisher_pubkey, &e.listing_id);
    let release = release_id(&Host, &listing, "qwen-small@1");
    assert_eq!((hex(&listing).as_str(), hex(&release).as_str()), (LISTING_KEY, RELEASE_KEY));
}
