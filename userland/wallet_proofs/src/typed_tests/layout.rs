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

use crate::typed::reply::parse_reply;
use crate::typed::*;

pub(crate) fn words(n: usize) -> Vec<[u8; 32]> {
    (0..n).map(|i| [i as u8 + 1; 32]).collect()
}

#[test]
fn request_layout() {
    let w = words(2);
    let r = typed_request(7, 9, ENTRY_ATTEST, 11155111, &w);
    assert_eq!(r.len(), 21 + 64);
    assert_eq!(&r[..4], &7u32.to_le_bytes());
    assert_eq!(&r[4..8], &9u32.to_le_bytes());
    assert_eq!(&r[8..10], &3u16.to_le_bytes());
    assert_eq!(&r[10..12], &[0, 0]);
    assert_eq!(&r[12..20], &11155111u64.to_le_bytes());
    assert_eq!(r[20], 2);
    assert_eq!(&r[21..53], &w[0]);
    assert_eq!(&r[53..], &w[1]);
    assert_eq!((OP_SIGN_TYPED_DATA, ENTRY_BINDING), (30, 2));
}

fn reply(signer: [u8; 20], sh: [u8; 32]) -> Vec<u8> {
    let mut b = signer.to_vec();
    b.extend_from_slice(&[0xd5; 32]);
    b.extend_from_slice(&sh);
    b.extend_from_slice(&[0xd9; 32]);
    b.extend_from_slice(&[0x5a; 65]);
    b
}

#[test]
fn reply_round_trip_and_length() {
    let b = reply([1; 20], [2; 32]);
    assert_eq!(b.len(), REPLY_LEN);
    let s = parse_reply(&b).unwrap();
    assert_eq!((s.signer, s.struct_hash, s.sig), ([1; 20], [2; 32], [0x5a; 65]));
    assert_eq!((s.domain_separator, s.digest), ([0xd5; 32], [0xd9; 32]));
    assert!(parse_reply(&b[..180]).is_none());
    assert!(parse_reply(&[b.as_slice(), &[0]].concat()).is_none());
}
