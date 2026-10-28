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

//! Way 3 of the key header (fs/blockfs_volume/key_wrap.rs): the volume key
//! wrapped under the approved chain's key, with the header's bytes 0..56 as
//! the associated data, so a relabelled or altered header does not unwrap.

use super::key_header::{parse_key_header, Keyed};
use super::key_seal::encode_key_header;
use super::key_wrap::{unwrap, wrap};

const KEK: [u8; 32] = [7; 32];
const KEY: [u8; 32] = [42; 32];

fn header() -> [u8; 512] {
    encode_key_header(&Keyed::Approved(wrap(&KEK, &KEY, [9; 12]).unwrap()))
}

fn opens(sector: &[u8; 512]) -> Option<[u8; 32]> {
    match parse_key_header(sector) {
        Ok(Some(Keyed::Approved(w))) => unwrap(&KEK, &w),
        _ => None,
    }
}

#[test]
fn a_wrapped_key_comes_back_through_its_header() {
    let s = header();
    assert_eq!(s[8], 3, "way 3");
    assert_eq!(opens(&s), Some(KEY));
}

#[test]
fn another_key_does_not_unwrap_it() {
    let Ok(Some(Keyed::Approved(w))) = parse_key_header(&header()) else { panic!("not way 3") };
    assert_eq!(unwrap(&[8; 32], &w), None);
}

#[test]
fn every_byte_the_header_is_read_by_is_bound() {
    // Bytes 9..12 are padding no way reads, so changing them changes nothing.
    let s = header();
    for at in (0..9).chain(12..116) {
        let mut bent = s;
        bent[at] ^= 1;
        assert_eq!(opens(&bent), None, "byte {at} changed and it still opened");
    }
}

#[test]
fn a_way_three_header_relabelled_as_a_passphrase_does_not_unseal() {
    let mut s = header();
    s[8] = 2;
    let Ok(Some(Keyed::Passphrase(sealed))) = parse_key_header(&s) else { panic!("not way 2") };
    assert_eq!(super::key_seal::unseal_key(&KEK, &sealed), None);
}
