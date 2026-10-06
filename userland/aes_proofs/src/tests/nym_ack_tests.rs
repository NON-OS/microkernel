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
//! Host proofs for the software AES.

//! net.nym's acknowledgement: what one ack seals, the reader opens to the
//! same fragment, and nothing else does.

use crate::ack::open::open_ack;
use crate::ack::plaintext::ack_plaintext;

const KEY: [u8; 16] = [0x42; 16];

#[test]
fn an_ack_opens_to_the_fragment_it_was_sealed_for() {
    for id in [[0, 0, 0, 1, 1], [0x7f, 0xff, 0xff, 0xff, 255], [0x12, 0x34, 0x56, 0x78, 18]] {
        let sealed = ack_plaintext(&KEY, id).unwrap();
        assert_ne!(&sealed[16..], &id[..], "the id is not carried in the clear");
        assert_eq!(open_ack(&KEY, &sealed), Some(id));
    }
}

#[test]
fn two_acks_for_one_fragment_share_no_bytes_a_mix_could_group() {
    let id = [1, 2, 3, 4, 5];
    let a = ack_plaintext(&KEY, id).unwrap();
    let b = ack_plaintext(&KEY, id).unwrap();
    assert_ne!(a, b);
    assert_eq!(open_ack(&KEY, &a), open_ack(&KEY, &b));
}

#[test]
fn an_ack_under_another_key_or_of_another_width_names_nothing_of_ours() {
    let id = [9, 9, 9, 9, 9];
    let sealed = ack_plaintext(&KEY, id).unwrap();
    assert_ne!(open_ack(&[0x43; 16], &sealed), Some(id));
    assert_eq!(open_ack(&KEY, &sealed[..20]), None);
    let mut long = sealed.to_vec();
    long.push(0);
    assert_eq!(open_ack(&KEY, &long), None);
}
