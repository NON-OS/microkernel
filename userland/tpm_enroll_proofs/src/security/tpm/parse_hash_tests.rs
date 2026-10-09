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

//! Hash tickets, against bytes written from the specification.

use super::parse_util::resp;
use crate::security::tpm::enroll::hash::{build_hash, parse_hash};
use crate::security::tpm::enroll::EnrollError;
use crate::security::tpm::enroll::AK_SIGN_LABEL;

fn ticket(tag: u16, hierarchy: u32, n: usize) -> Vec<u8> {
    [&tag.to_be_bytes()[..], &hierarchy.to_be_bytes(), &(n as u16).to_be_bytes(), &vec![7; n]]
        .concat()
}

fn hashed(t: &[u8]) -> Vec<u8> {
    resp(0, &[&[0, 32][..], &[5; 32], t].concat())
}

#[test]
fn hash_runs_under_the_endorsement_hierarchy() {
    let cmd = build_hash(&[3; 32]);
    let data = [AK_SIGN_LABEL, &[3; 32]].concat();
    assert_eq!(cmd[6..12], [0, 0, 0x01, 0x7D, 0, data.len() as u8]);
    assert_eq!(cmd[12..12 + data.len()], data[..], "the label, then the message");
    assert_eq!(cmd[12 + data.len()..], [0x00, 0x0B, 0x40, 0x00, 0x00, 0x0B]);
}

#[test]
fn a_null_or_malformed_ticket_is_refused() {
    let good = ticket(0x8024, 0x4000_000B, 64);
    let (digest, t) = parse_hash(&hashed(&good)).expect("a ticket");
    assert_eq!((digest, t), ([5; 32], good.clone()));
    let null = parse_hash(&hashed(&ticket(0x8024, 0x4000_0007, 0))).err();
    assert_eq!(null, Some(EnrollError::Unsignable));
    for bad in [
        ticket(0x8021, 0x4000_000B, 32),
        ticket(0x8024, 0x4000_0001, 32),
        ticket(0x8024, 0x4000_000B, 65),
    ] {
        assert!(parse_hash(&hashed(&bad)).is_err(), "{bad:02x?}");
    }
    let whole = hashed(&good);
    for cut in 0..whole.len() {
        assert!(parse_hash(&whole[..cut]).is_err(), "cut at {cut}");
    }
}
