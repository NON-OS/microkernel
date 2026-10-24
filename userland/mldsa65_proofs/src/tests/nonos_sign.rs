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

//! Agreement with nonos-sign, the host build of the very C source the
//! kernel and the loader compile. vectors/nonos_sign.txt was written by
//! nonos-sign: a key pair made with zero randomness, a random key pair, and
//! signatures of four messages with zero randomness under each. The crate
//! must give the same bytes from the same inputs, and verify all of them.

use crate::hex::de;
use nonos_mldsa65::{keypair, sign, verify, SecretKey, PUBLIC_LEN};

fn zeros(b: &mut [u8]) -> bool {
    b.fill(0);
    true
}

fn messages() -> [Vec<u8>; 4] {
    [vec![], b"nsh/1".to_vec(), vec![0xA5; 1000], vec![7; 65537]]
}

fn key(pk: &str, sk: &str) -> ([u8; PUBLIC_LEN], SecretKey) {
    (de(pk).try_into().unwrap(), SecretKey::from_bytes(&de(sk).try_into().unwrap()))
}

#[test]
fn every_vector_from_nonos_sign_is_reproduced() {
    let text = include_str!("../../vectors/nonos_sign.txt");
    let lines: Vec<Vec<&str>> = text.lines().map(|l| l.split(' ').collect()).collect();
    assert_eq!(lines.len(), 6);
    let (zpk, zsk) = key(lines[0][1], lines[0][2]);
    let (made_pk, made_sk) = keypair(&mut zeros).unwrap();
    assert_eq!(
        (made_pk, made_sk.as_bytes()),
        (zpk, zsk.as_bytes()),
        "the zero-randomness key pair"
    );
    let (rpk, rsk) = key(lines[1][1], lines[1][2]);
    for (i, m) in messages().iter().enumerate() {
        let l = &lines[2 + i];
        assert_eq!((l[1], l[2]), (i.to_string().as_str(), m.len().to_string().as_str()));
        for (pk, sk, want) in [(&zpk, &zsk, l[3]), (&rpk, &rsk, l[4])] {
            let want: [u8; 3309] = de(want).try_into().unwrap();
            assert_eq!(sign(sk, m, &mut zeros).unwrap(), want, "message {i}");
            assert!(verify(pk, m, &want), "message {i}");
        }
    }
}
