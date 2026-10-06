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

//! Known answers for the frozen constants, from an independent implementation
//! written from the specification with BLAKE3 taken from the b3sum binary. Do
//! not re-baseline: a change here moves every commitment, tag and leaf.

use stark_proofs::crypto::stark::field::Fp;

use crate::domain::{DEVICE_DOMAIN, TAG_DOMAIN};
use crate::native::{commit, leaf, scope, tag};

fn words(v: [u64; 4]) -> [Fp; 4] {
    v.map(Fp::from_u64)
}

#[test]
fn the_domains_are_their_ascii_names() {
    assert_eq!(DEVICE_DOMAIN.to_be_bytes(), *b"NONOSDV1");
    assert_eq!(TAG_DOMAIN.to_be_bytes(), *b"NONOSTG1");
}

#[test]
fn a_device_commitment_holds() {
    let s = words([0x1111, 0x2222, 0x3333, 0x4444]);
    let want =
        [12498542759306190300, 8155002663235667937, 11205017779145142187, 6783626904244838236];
    assert_eq!(commit(&s), words(want));
}

#[test]
fn a_scope_and_its_tag_hold() {
    let e = scope(b"faucet.nonos.software", 20_000).expect("scope");
    assert_eq!(e, [Fp::from_u64(2476678180905343861), Fp::from_u64(3835201175105680633)]);
    let s = words([0x1111, 0x2222, 0x3333, 0x4444]);
    let want =
        [9719737796216736658, 17411401048696214809, 2084586678860056346, 4791691562596953882];
    assert_eq!(tag(&s, &e), words(want));
}

/// One full leaf of each kind over the digest 0x00..0x1f: kernel, capsule, pad
/// and bootloader, the four the trees hold.
#[test]
fn a_leaf_of_each_kind_holds() {
    let d: [u8; 32] = core::array::from_fn(|i| i as u8);
    let want: [[u64; 4]; 4] = [
        [3052267470148017272, 14687556820788697121, 15286552794856528800, 14246847108617681346],
        [14841500172566672250, 12224552880640628080, 6776997464967757589, 7730051411625761763],
        [17597477727678947987, 11422089482064560314, 10688476522823791558, 16619848848565472881],
        [15789660095731822450, 14193700703793183257, 14785613981763072418, 5113536150356216535],
    ];
    for (kind, w) in want.iter().enumerate() {
        assert_eq!(leaf(kind as u64, &d), words(*w), "kind {kind}");
    }
}
