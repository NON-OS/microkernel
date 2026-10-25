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

use alloc::vec;

use nonos_marketplace_abi::decode_index;

use super::check::check;
use super::seeds::seeds;

#[test]
fn the_encoders_blobs_decode_back() {
    for (blob, listings) in seeds().iter().zip([0usize, 1, 3]) {
        assert!(check(blob));
        let Ok(d) = decode_index(blob) else { panic!("an encoded index decodes") };
        assert_eq!(d.index.entries.len(), listings);
        assert_eq!(d.index.index_signature, vec![7u8; 64]);
        assert_eq!(d.signed_bytes.len(), blob.len() - 4 - 64);
    }
}
