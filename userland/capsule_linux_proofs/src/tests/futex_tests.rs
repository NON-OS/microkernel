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

//! futex decodes and refuses as Linux's do_futex does, reads its counts as
//! the ints they are, and a wake takes only waiters whose bitset meets its
//! own, so it can never be used up on a waiter it was not meant for.

use super::random::Regs;
use crate::linux::abi::errno::{EINVAL, ENOSYS};
use crate::linux::call::futex_op::{decode, requeue_counts, wake_count, Cmd};
use crate::linux::guest::futex_pick::{pick, MATCH_ANY};

const WAIT: u64 = 0;
const WAKE: u64 = 1;
const REQUEUE: u64 = 3;
const CMP_REQUEUE: u64 = 4;
const WAIT_BITSET: u64 = 9;
const WAKE_BITSET: u64 = 10;
const PRIVATE: u64 = 128;
const REALTIME: u64 = 256;
const W: u64 = 0x7000_1000;

/// uaddr, op, uaddr2, val3, and what the decoder answers.
type Row = (u64, u64, u64, u64, Result<Cmd, i64>);

#[test]
fn each_operation_decodes_or_is_refused_with_linux_errno() {
    let table: [Row; 17] = [
        (W, WAIT, 0, 0, Ok(Cmd::Wait { bits: MATCH_ANY, absolute: false })),
        (W, WAIT | PRIVATE, 0, 7, Ok(Cmd::Wait { bits: MATCH_ANY, absolute: false })),
        (W, WAIT_BITSET | PRIVATE | REALTIME, 0, 6, Ok(Cmd::Wait { bits: 6, absolute: true })),
        (W, WAKE | PRIVATE, 0, 0, Ok(Cmd::Wake { bits: MATCH_ANY })),
        (W, WAKE_BITSET, 0, 0x8000_0000, Ok(Cmd::Wake { bits: 0x8000_0000 })),
        (W, REQUEUE, W + 4, 0, Ok(Cmd::Requeue { compare: false })),
        (W, CMP_REQUEUE | PRIVATE, W, 0, Ok(Cmd::Requeue { compare: true })),
        // A zero bitset matches nothing and is refused.
        (W, WAIT_BITSET, 0, 0, Err(EINVAL)),
        (W, WAKE_BITSET, 0, 1 << 32, Err(EINVAL)),
        // The realtime clock is WAIT_BITSET's alone.
        (W, WAIT | REALTIME, 0, 0, Err(ENOSYS)),
        (W, WAKE | REALTIME, 0, 0, Err(ENOSYS)),
        // Operations not served, and bits outside the command, are ENOSYS:
        // 0x200 is not FUTEX_WAIT with a stray bit.
        (W, 5, 0, 0, Err(ENOSYS)),
        (W, 6 | PRIVATE, 0, 0, Err(ENOSYS)),
        (W, 0x200, 0, 0, Err(ENOSYS)),
        // A word is four bytes on a four-byte boundary, both of them.
        (W + 2, WAIT, 0, 0, Err(EINVAL)),
        (W + 1, WAKE, 0, 0, Err(EINVAL)),
        (W, CMP_REQUEUE, W + 3, 0, Err(EINVAL)),
    ];
    for (uaddr, op, uaddr2, val3, want) in table {
        assert_eq!(decode(uaddr, op, uaddr2, val3), want, "{uaddr:#x} op {op:#x}");
    }
}

#[test]
fn a_wake_count_is_an_int_and_wakes_at_least_one() {
    assert_eq!(wake_count(0), 1, "Linux wakes one before it compares");
    assert_eq!(wake_count(1), 1);
    assert_eq!(wake_count(5), 5);
    assert_eq!(wake_count(i32::MAX as u64), i32::MAX as u64);
    assert_eq!(wake_count(u32::MAX as u64), 1, "-1 as an int");
    assert_eq!(wake_count(u64::MAX), 1, "-1 sign-extended by the caller");
    assert_eq!(wake_count((1 << 32) | 3), 3, "the upper half is not the int's");
}

#[test]
fn requeue_counts_are_ints_and_a_negative_one_is_refused() {
    assert_eq!(requeue_counts(1, 2), Ok((1, 2)));
    assert_eq!(requeue_counts(0, 0), Ok((0, 0)));
    assert_eq!(requeue_counts(u64::MAX, 0), Err(EINVAL));
    assert_eq!(requeue_counts(0, 0x8000_0000), Err(EINVAL));
    assert_eq!(requeue_counts(i32::MAX as u64, (1 << 32) | 1), Ok((i32::MAX as u64, 1)));
}

#[test]
fn a_wake_takes_only_waiters_whose_bitset_meets_its_own() {
    let (a, b) = (W, W + 4);
    let waits = [(1, a), (2, b), (3, a), (4, a)];
    let bits = |tid: u32| match tid {
        1 => 0b01,
        3 => 0b10,
        _ => MATCH_ANY,
    };
    // A wake for bit 1 passes over thread 1 and takes thread 3, the one it
    // was meant for. Taking thread 1 instead would leave 3 parked forever.
    assert_eq!(pick(&waits, bits, a, 0b10, 1), [2]);
    assert_eq!(pick(&waits, bits, a, 0b01, 9), [0, 3]);
    assert_eq!(pick(&waits, bits, a, 0b100, 9), [3]);
    assert_eq!(pick(&waits, bits, a, MATCH_ANY, 2), [0, 2]);
    assert_eq!(pick(&waits, bits, b, MATCH_ANY, 9), [1]);
    assert_eq!(pick(&waits, bits, W + 8, MATCH_ANY, 9), [] as [usize; 0]);
    assert_eq!(pick(&waits, bits, a, MATCH_ANY, 0), [] as [usize; 0]);
}

/// Random waiters, words, bitsets and counts: what is picked is in order, on
/// the word, meets the bitset, is no more than asked, and passes over no
/// waiter that would have qualified while the count still had room.
#[test]
fn random_wakes_pick_exactly_the_waiters_linux_would() {
    let mut r = Regs::new(0x6675_7465_785f_7069);
    for _ in 0..20_000 {
        let waits: Vec<(u32, u64)> =
            (0..r.small(12)).map(|i| (i as u32 + 1, W + 4 * r.small(3))).collect();
        let masks: Vec<u32> = (0..13).map(|_| r.any() as u32 | 1 << r.small(32)).collect();
        let bits_of = |tid: u32| masks[tid as usize];
        let (word, bits, count) = (W + 4 * r.small(3), r.any() as u32 | 1, r.small(6));
        let got = pick(&waits, bits_of, word, bits, count);
        assert!(got.len() as u64 <= count && got.windows(2).all(|p| p[0] < p[1]));
        let fits = |i: usize| waits[i].1 == word && bits_of(waits[i].0) & bits != 0;
        assert!(got.iter().all(|&i| fits(i)));
        let skipped = (0..waits.len()).filter(|&i| fits(i) && !got.contains(&i)).count();
        assert!(skipped == 0 || got.len() as u64 == count, "a fitting waiter was passed over");
        let op = r.arg();
        if let Ok(Cmd::Wait { bits, .. } | Cmd::Wake { bits }) =
            decode(r.arg(), op, r.arg(), r.arg())
        {
            assert_ne!(bits, 0);
        }
    }
}
