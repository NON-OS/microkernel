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

//! Every plan stays inside the limits, the descriptor and dwords are laid
//! out as the spec lays them out, and the rules that keep the extras from
//! costing the disk.

use crate::admin::hmb::{descriptor, enable_dwords, ends_attempt, extras_allowed, plan};
use crate::admin::hmb::{ENABLE_TIMEOUT_MS, MAX_DESCRIPTORS, QUEUES_TIMEOUT_MS};
use crate::error::NvmeError;
use crate::hmb_plan_tests::ask;

#[test]
fn every_ask_gives_a_plan_inside_the_limits() {
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..20_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let (pre, min) = (s as u32 % 70_000, (s >> 20) as u32 % 40_000);
        let a = ask(pre, min, (s >> 40) as u32 % 3000, (s >> 52) as u16 % 300);
        if let Some(p) = plan(a) {
            assert!(p.piece_pages >= 1 && p.piece_pages <= 1024);
            assert!(p.pieces >= 1 && p.pieces <= MAX_DESCRIPTORS);
            assert!(a.max_pieces == 0 || p.pieces <= a.max_pieces as u32);
            assert!(p.piece_pages >= a.min_piece);
            assert!(p.pages() >= a.minimum);
            assert!(p.pages() <= 32_768 + 1024);
        }
    }
}

#[test]
fn the_descriptor_and_dwords_are_as_the_spec_lays_them_out() {
    let d = descriptor(0x0000_0001_2345_6000, 1024);
    assert_eq!(&d[0..8], &0x0000_0001_2345_6000u64.to_le_bytes());
    assert_eq!(&d[8..12], &1024u32.to_le_bytes());
    assert_eq!(&d[12..16], &[0; 4]);
    assert_eq!(enable_dwords(16_384, 0x0000_0002_0000_1000, 16), [1, 16_384, 0x1000, 2, 16]);
}

#[test]
fn a_failed_attempt_turns_the_extras_off_for_good() {
    assert!(extras_allowed(false));
    assert!(!extras_allowed(true));
    // The buffer's enable gets far longer than an identify, and the queue
    // count no longer than one.
    const { assert!(ENABLE_TIMEOUT_MS >= 30_000) };
    const { assert!(QUEUES_TIMEOUT_MS <= 5_000) };
}

#[test]
fn only_a_refusal_is_carried_on_from() {
    assert!(!ends_attempt(NvmeError::AdminCommandFailed));
    assert!(ends_attempt(NvmeError::ControllerTimeout));
    assert!(ends_attempt(NvmeError::ClockFailed));
    assert!(ends_attempt(NvmeError::ControllerFatal));
}
