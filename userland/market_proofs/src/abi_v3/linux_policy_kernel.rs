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

//! The kernel keeps its own copy of OP_GET_LINUX_POLICY's wire (plan/41);
//! this holds it to market_proto's, both ways: every reply the market writes
//! the kernel reads the same, and every byte string market_proto refuses
//! the kernel refuses, over every length a reply could take.

use nonos_market_proto::linux_policy::{MARKET_ARGS_MAX, MARKET_ARG_LEN};
use nonos_market_proto::{linux_policy_body, parse_linux_policy, LinuxPolicy};

use super::kernel_linux_policy::{self as kernel, parse, LinuxPolicyReply};

type Proto = Option<(LinuxPolicy, [u8; 32])>;
type Kernel = Option<(LinuxPolicyReply, [u8; 32])>;

fn same(proto: Proto, kernel: Kernel) -> bool {
    match (proto, kernel) {
        (None, None) => true,
        (Some((p, a)), Some((k, b))) if a == b => match (p, k) {
            (LinuxPolicy::NoPolicy, LinuxPolicyReply::NoPolicy) => true,
            (LinuxPolicy::Refused, LinuxPolicyReply::Refused) => true,
            (LinuxPolicy::Policy(a, x), LinuxPolicyReply::Policy(b, y)) => a == b && x == y,
            _ => false,
        },
        _ => false,
    }
}

const HASH: [u8; 32] = [5; 32];

#[test]
fn the_kernel_reads_every_reply_the_market_writes() {
    assert!((kernel::MARKET_ARG_LEN, kernel::MARKET_ARGS_MAX) == (MARKET_ARG_LEN, MARKET_ARGS_MAX));
    let m: [u8; MARKET_ARG_LEN] = core::array::from_fn(|i| (i * 7 % 256) as u8);
    let p = |n| LinuxPolicy::Policy(Box::new(m), vec![3; n]);
    for reply in [LinuxPolicy::NoPolicy, LinuxPolicy::Refused, p(0), p(1), p(MARKET_ARGS_MAX)] {
        let body = linux_policy_body(&reply, &HASH);
        let read = parse(&body);
        assert!(read.as_ref().is_some_and(|(_, h)| *h == HASH));
        assert!(same(parse_linux_policy(&body), read));
    }
}

#[test]
fn the_two_copies_agree_on_every_short_body_and_every_length() {
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            for tail in [0usize, 1, 31, 32, 33] {
                let body = [&[a, b][..], &vec![0u8; tail]].concat();
                assert!(same(parse_linux_policy(&body), parse(&body)), "{body:?}");
            }
        }
        assert!(same(parse_linux_policy(&[a]), parse(&[a])));
    }
    for kind in 0..=3u8 {
        for len in 0..=(MARKET_ARG_LEN + MARKET_ARGS_MAX + 4) {
            let body = [&[2u8, kind][..], &HASH, &vec![9u8; len]].concat();
            assert!(same(parse_linux_policy(&body), parse(&body)), "kind {kind}, {len} bytes");
        }
    }
}
