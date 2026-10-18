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

use nonos_proof_abi::{parse, tag};

use super::container_tests::sample;

#[test]
fn a_trailing_or_missing_byte_is_refused() {
    let b = sample();
    assert!(parse(&[&b[..], &[0]].concat()).is_none());
    assert!(parse(&b[..b.len() - 1]).is_none());
}

#[test]
fn a_section_length_that_runs_past_the_end_is_refused() {
    let mut b = sample();
    let at = 20 + 6 + 240 + 2;
    b[at..at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(parse(&b).is_none());
}

#[test]
fn a_tag_the_kind_does_not_carry_there_is_refused() {
    for t in [tag::RECORD, tag::ATTEST, tag::DELIVERY] {
        let mut b = sample();
        b[20 + 6 + 240] = t as u8;
        assert!(parse(&b).is_none(), "tag {t} in the device's place");
    }
}
