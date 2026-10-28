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

//! OP_GET_LINUX_POLICY's reply, listed arguments and all, reads back
//! exactly; a byte short or past, another version or kind, is no reply.

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::*;

fn arg() -> [u8; MARKET_ARG_LEN] {
    core::array::from_fn(|i| u8::try_from(i % 251).unwrap_or(0))
}

fn p(n: usize) -> LinuxPolicy {
    LinuxPolicy::Policy(Box::new(arg()), alloc::vec![9; n])
}

const HASH: [u8; 32] = [7; 32];

#[test]
fn every_kind_reads_back_as_written_with_its_release_s_hash() {
    let all = [LinuxPolicy::NoPolicy, LinuxPolicy::Refused, p(0), p(1), p(MARKET_ARGS_MAX)];
    for reply in all {
        assert_eq!(parse_linux_policy(&linux_policy_body(&reply, &HASH)), Some((reply, HASH)));
    }
}

#[test]
fn a_cut_long_or_foreign_reply_is_refused() {
    let full = linux_policy_body(&p(MARKET_ARGS_MAX), &HASH);
    let none = linux_policy_body(&LinuxPolicy::NoPolicy, &HASH);
    let cut = full.get(..LINUX_POLICY_HEAD + MARKET_ARG_LEN - 1).expect("long enough");
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
    assert_eq!(linux_policy_body(&p(0), &HASH).len(), 354);
    assert_eq!(linux_policy_body(&LinuxPolicy::Refused, &HASH).len(), LINUX_POLICY_HEAD);
    assert_eq!((KIND_NONE, KIND_POLICY, KIND_REFUSED, LINUX_POLICY_VERSION), (0, 1, 2, 2));
}
