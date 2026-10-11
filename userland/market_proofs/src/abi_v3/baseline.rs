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

//! The schema 2 catalogue this tree ships still reads, as schema 2: nothing
//! in it can be bought, every release keeps its v2 signing domain, and it
//! cannot be re-encoded until each entry gains a commerce block.

use nonos_marketplace_abi::SCHEMA_V2;
use nonos_marketplace_abi::{decode_index, encode_index, release_signing_bytes, EncodeError};

static BASELINE: &[u8] = include_bytes!("../../../../nonos-data/market/index.bin");

#[test]
fn the_shipped_schema_2_catalogue_still_decodes() {
    let d = decode_index(BASELINE).expect("the shipped catalogue decodes");
    assert_eq!(d.index.schema_version, SCHEMA_V2);
    assert!(!d.index.entries.is_empty());
    for e in &d.index.entries {
        assert!(e.commerce.is_none(), "{}", e.listing_id);
        for r in &e.releases {
            assert!(r.ext.is_none());
            let signed = release_signing_bytes(r);
            assert_eq!(&signed[4..32], b"NONOS.marketplace.release.v2");
        }
    }
    assert_eq!(encode_index(&d.index).err(), Some(EncodeError::MissingCommerce));
}
