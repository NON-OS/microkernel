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

//! The certificate index's public area, its chunks, and the trim to DER.

use super::parse_util::resp;
use crate::security::tpm::enroll::cert::der_trim;
use crate::security::tpm::enroll::nv_public::{parse_nv_read_public, NvSlot};
use crate::security::tpm::enroll::nv_read::parse_nv_read;
use crate::security::tpm::enroll::EnrollError;

const INDEX: u32 = 0x01C0_0002;
const AUTHREAD: u32 = 1 << 18;
const OWNERREAD: u32 = 1 << 17;
const WRITTEN: u32 = 1 << 29;

fn nv_public(index: u32, attrs: u32, size: u16) -> Vec<u8> {
    let p =
        [&index.to_be_bytes()[..], &[0, 0x0B], &attrs.to_be_bytes(), &[0, 0], &size.to_be_bytes()];
    let p = p.concat();
    resp(0, &[&(p.len() as u16).to_be_bytes()[..], &p, &[0, 34], &[1; 34]].concat())
}

#[test]
fn the_certificate_index_is_read_as_the_profile_allows() {
    let parse = |a: u32, n: u16| parse_nv_read_public(&nv_public(INDEX, a, n), INDEX);
    let both = AUTHREAD | OWNERREAD | WRITTEN;
    assert_eq!(parse(both, 1016), Ok(NvSlot { size: 1016, auth: INDEX }));
    assert_eq!(parse(OWNERREAD | WRITTEN, 4096), Ok(NvSlot { size: 4096, auth: 0x4000_0001 }));
    assert_eq!(parse(both, 4097), Err(EnrollError::OutOfBounds));
    for (attrs, n) in [(WRITTEN, 9), (AUTHREAD, 9), (both | 0x10, 9), (both, 0)] {
        assert!(parse(attrs, n).is_err(), "{attrs:#x} {n}");
    }
    assert!(parse_nv_read_public(&nv_public(INDEX + 1, both, 9), INDEX).is_err());
    let whole = nv_public(INDEX, both, 1016);
    for cut in 0..whole.len() - 36 {
        assert!(parse_nv_read_public(&whole[..cut], INDEX).is_err(), "cut at {cut}");
    }
}

#[test]
fn an_nv_chunk_is_exactly_what_was_asked_for() {
    let r = resp(0, &[&[0, 0, 0, 0, 0, 5][..], b"chunk"].concat());
    assert_eq!(parse_nv_read(&r, 5).expect("five bytes"), b"chunk");
    assert!(parse_nv_read(&r, 6).is_err());
    assert!(parse_nv_read(&r[..r.len() - 1], 5).is_err());
}

#[test]
fn a_certificate_is_trimmed_to_its_der_length() {
    let long = [&[0x30, 0x82, 0x01, 0x00][..], &[4; 256], &[0xFF; 64]].concat();
    assert_eq!(der_trim(long.clone()).expect("long form").len(), 260);
    let short = [&[0x30, 0x81, 0x10][..], &[4; 16], &[0xFF; 8]].concat();
    assert_eq!(der_trim(short).expect("short form").len(), 19);
    for bad in [&long[..259], &[0x31, 0x82, 1, 0][..], &[0x30, 0x83, 0, 1, 0][..], &[0x30][..]] {
        assert!(der_trim(bad.to_vec()).is_err(), "{:02x?}", &bad[..bad.len().min(4)]);
    }
}
