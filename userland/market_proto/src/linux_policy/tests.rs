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

//! OP_GET_LINUX_POLICY's reply reads back exactly, and anything else, a
//! byte short or past, another version or kind, is no reply.

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::*;

fn arg() -> [u8; MARKET_ARG_LEN] {
    let mut m = [0u8; MARKET_ARG_LEN];
    for (i, b) in m.iter_mut().enumerate() {
        *b = u8::try_from(i % 251).unwrap_or(0);
    }
    m
}

const HASH: [u8; 32] = [7; 32];

#[test]
fn every_kind_reads_back_as_written_with_its_release_s_hash() {
    for reply in [LinuxPolicy::NoPolicy, LinuxPolicy::Refused, LinuxPolicy::Policy(Box::new(arg()))]
    {
        assert_eq!(parse_linux_policy(&linux_policy_body(&reply, &HASH)), Some((reply, HASH)));
    }
}

#[test]
fn a_cut_long_or_foreign_reply_is_refused() {
    let full = linux_policy_body(&LinuxPolicy::Policy(Box::new(arg())), &HASH);
    let none = linux_policy_body(&LinuxPolicy::NoPolicy, &HASH);
    let (_, cut) = full.split_last().expect("not empty");
    let (_, cut_none) = none.split_last().expect("not empty");
    let foreign: Vec<u8> =
        full.iter().enumerate().map(|(i, b)| if i == 0 { 1 } else { *b }).collect();
    let bad: Vec<Vec<u8>> = alloc::vec![
        cut.to_vec(),
        cut_none.to_vec(),
        [full.as_slice(), &[0]].concat(),
        [none.as_slice(), &[0]].concat(),
        foreign,
        [&[LINUX_POLICY_VERSION, 3][..], &HASH].concat(),
        [&[1, KIND_NONE][..], &HASH].concat(),
        alloc::vec![LINUX_POLICY_VERSION, KIND_NONE],
        alloc::vec![LINUX_POLICY_VERSION, KIND_POLICY],
        alloc::vec![LINUX_POLICY_VERSION],
        Vec::new(),
    ];
    for b in bad {
        assert_eq!(parse_linux_policy(&b), None, "{} bytes", b.len());
    }
}

#[test]
fn the_op_and_lengths_are_pinned() {
    assert_eq!(crate::ops::OP_GET_LINUX_POLICY, 8);
    assert_eq!(linux_policy_body(&LinuxPolicy::Policy(Box::new(arg())), &HASH).len(), 354);
    assert_eq!(linux_policy_body(&LinuxPolicy::Refused, &HASH).len(), LINUX_POLICY_HEAD);
    assert_eq!((KIND_NONE, KIND_POLICY, KIND_REFUSED, LINUX_POLICY_VERSION), (0, 1, 2, 2));
}
