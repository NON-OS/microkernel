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

//! The voucher's words, struct hash and keyring request, each
//! against an encoding built here by hand from plan/20 and plan/31.

mod host;

use host::{keccak, Host};
use nonos_nox_meter::voucher::Voucher;

fn voucher() -> Voucher {
    Voucher {
        channel_id: [1; 32],
        payer: [0xaa; 20],
        payee: [0xbb; 20],
        listing_id: [2; 32],
        unit: [3; 32],
        cumulative: 0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10,
        seq: 7,
        expiry: 1_900_000_000,
        usage_root: [4; 32],
    }
}

#[test]
fn the_struct_hash_is_eip_712_built_by_hand() {
    let v = voucher();
    let ty = keccak(b"Voucher(bytes32 channelId,address payer,address payee,bytes32 listingId,bytes32 unit,uint256 cumulative,uint64 seq,uint64 expiry,bytes32 usageRoot)");
    let mut pre = ty.to_vec();
    for w in v.words() {
        pre.extend_from_slice(&w);
    }
    assert_eq!(v.struct_hash(&Host), keccak(&pre));
    let w = v.words();
    assert_eq!((&w[1][..12], &w[1][12..]), (&[0u8; 12][..], &[0xaa; 20][..]));
    assert_eq!(&w[5][..16], &[0u8; 16]);
    assert_eq!(&w[5][16..], &v.cumulative.to_be_bytes());
    assert_eq!(&w[6][..24], &[0u8; 24]);
    assert_eq!(w[6][31], 7);
}
