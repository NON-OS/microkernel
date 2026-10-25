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

//! OP_GET_RELEASE's escrow fields (B8, v1.4.36): the market appends the
//! chunk root and length after the validation time, and the wallet reads
//! them with the package url, from the bytes the market writes.

use nonos_market_proto::{parse_release, parse_release_escrow, ReleaseEscrow};
use nonos_marketplace_abi::ReleaseExt;

use crate::readiness_tests::fixture::release;
use crate::release_wire::encode;

fn escrowed() -> nonos_marketplace_abi::CapsuleRelease {
    let mut rel = release("x86_64-linux", 0);
    let ext = rel.ext.as_mut().unwrap();
    (ext.chunk_root, ext.content_len) = ([0x5A; 32], 70_000);
    rel
}

#[test]
fn the_escrow_fields_follow_the_validation_time() {
    let body = encode(&escrowed());
    let e = parse_release_escrow(&body).unwrap();
    let want = ReleaseEscrow {
        package_url: b"http://mirror/pkg-1.apk".to_vec(),
        chunk_root: [0x5A; 32],
        content_len: 70_000,
    };
    assert_eq!(e, want);
    assert_eq!(body[body.len() - 8..], 70_000u64.to_le_bytes());
    assert!(parse_release(&body).is_some(), "the old reader still reads it");
}

#[test]
fn a_release_not_escrowed_reads_zero_and_a_cut_or_long_reply_is_refused() {
    let mut rel = escrowed();
    rel.ext = None;
    let e = parse_release_escrow(&encode(&rel)).unwrap();
    assert_eq!((e.chunk_root, e.content_len), ([0; 32], 0));
    rel.ext = Some(ReleaseExt { chunk_root: [0; 32], content_len: 0, ..escrowed().ext.unwrap() });
    assert_eq!(parse_release_escrow(&encode(&rel)).unwrap().content_len, 0);
    let body = encode(&escrowed());
    for n in 0..body.len() {
        assert!(parse_release_escrow(&body[..n]).is_none(), "cut at {n}");
    }
    let long = [&body[..], &[0]].concat();
    assert!(parse_release_escrow(&long).is_none(), "a byte past the fields");
}
