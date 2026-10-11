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

use nonos_proof_abi::{encode, parse, tag, Kind};

fn sample() -> Vec<u8> {
    let rec = [0u8; 240];
    encode(Kind::Isolation, &[(tag::RECORD, &rec), (tag::DEVICE, b"d"), (tag::ATTEST, b"t")])
        .expect("a container")
}

#[test]
fn a_trailing_or_missing_byte_is_refused() {
    let b = sample();
    assert!(parse(&[&b[..], &[0]].concat()).is_none());
    assert!(parse(&b[..b.len() - 1]).is_none());
}

#[test]
fn a_section_length_that_runs_past_the_end_is_refused() {
    let mut b = sample();
    let at = 12 + 6 + 240 + 2;
    b[at..at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(parse(&b).is_none());
}

#[test]
fn a_tag_out_of_order_in_the_bytes_is_refused() {
    let mut b = sample();
    let second = 12 + 6 + 240;
    b[second] = tag::RECORD as u8;
    assert!(parse(&b).is_none());
}
