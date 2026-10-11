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

//! The keyring request and reply of plan/31, byte for byte.

use nonos_nox_meter::typed::{parse_reply, voucher_request, REPLY_LEN, VOUCHER_ENTRY};
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
fn the_request_is_plan_31_byte_for_byte() {
    let v = voucher();
    let r = voucher_request(0x11223344, 5, 1, &v);
    assert_eq!(r.len(), 4 + 4 + 2 + 2 + 8 + 1 + 9 * 32);
    assert_eq!(&r[..4], &0x11223344u32.to_le_bytes());
    assert_eq!(&r[4..8], &5u32.to_le_bytes());
    assert_eq!(&r[8..10], &VOUCHER_ENTRY.to_le_bytes());
    assert_eq!(&r[10..12], &[0, 0]);
    assert_eq!(&r[12..20], &1u64.to_le_bytes());
    assert_eq!(r[20], 9);
    assert_eq!(&r[21..], &v.words().concat()[..]);
}

#[test]
fn a_reply_is_exactly_181_bytes() {
    let mut b = vec![0u8; REPLY_LEN];
    b[..20].copy_from_slice(&[0xaa; 20]);
    b[180] = 28;
    let s = parse_reply(&b).expect("parses");
    assert_eq!((s.signer, s.sig[64]), ([0xaa; 20], 28));
    assert!(parse_reply(&b[..180]).is_none());
    b.push(0);
    assert!(parse_reply(&b).is_none());
}
