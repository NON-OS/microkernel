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

//! poll and select take their counts as Linux reads them and refuse what it
//! refuses, select names a closed descriptor EBADF and narrows its sets
//! without reading past them, and epoll reports level and edge as Linux
//! does, allows EPOLLEXCLUSIVE where it does, and refuses a loop of epolls.

use super::random::Regs;
use crate::linux::abi::errno::EINVAL;
use crate::linux::file::epoll_rules::{exclusive_ok, live, loops, seen, MAX_NESTS};
use crate::linux::guest::{EPOLLET, EPOLLONESHOT};
use crate::linux::net::ready_sets::{
    first_closed, is_set, narrow, poll_count, select_count, set_bytes,
};

const MOST: u64 = 256;
const IN: u32 = 0x001;
const OUT: u32 = 0x004;
const HUP: u32 = 0x010;
const EXCLUSIVE: u32 = 1 << 28;
const ADD: u64 = 1;
const DEL: u64 = 2;
const MOD: u64 = 3;

/// op, events, whether the target is an epoll, the entry there, and the answer.
type ExclusiveRow = (u64, u32, bool, Option<u32>, Result<(), i64>);

#[test]
fn poll_counts_an_unsigned_int_no_larger_than_rlimit_nofile() {
    assert_eq!(poll_count(0, MOST), Ok(0));
    assert_eq!(poll_count(MOST, MOST), Ok(MOST));
    assert_eq!(poll_count(MOST + 1, MOST), Err(EINVAL));
    assert_eq!(poll_count(u32::MAX as u64, MOST), Err(EINVAL));
    assert_eq!(poll_count(u64::MAX, MOST), Err(EINVAL));
    assert_eq!(poll_count((1 << 32) | 3, MOST), Ok(3), "the upper half is not the count's");
}

#[test]
fn select_counts_an_int_refuses_a_negative_one_and_looks_no_further_than_the_table() {
    assert_eq!(select_count(u64::MAX, MOST), Err(EINVAL));
    assert_eq!(select_count(0x8000_0000, MOST), Err(EINVAL));
    assert_eq!(select_count(1024, MOST), Ok(MOST), "a bigger set is fine, as on Linux");
    assert_eq!(select_count(5, MOST), Ok(5));
    assert_eq!(select_count((1 << 32) | 5, MOST), Ok(5));
    assert_eq!([0, 1, 64, 65, 256].map(set_bytes), [0, 8, 8, 16, 32]);
}

#[test]
fn select_names_the_first_closed_descriptor_in_any_set() {
    let mut read = [0u8; 8];
    let mut write = [0u8; 8];
    read[0] = 0b0000_0110; // 1 and 2
    write[0] = 0b0010_0000; // 5
    let open = |fd: u64| fd < 4;
    assert_eq!(first_closed(&[&read, &write], 64, open), Some(5));
    assert_eq!(first_closed(&[&read], 64, open), None);
    // Only what nfds covers is looked at.
    assert_eq!(first_closed(&[&read, &write], 5, open), None);
}

#[test]
fn select_narrows_a_set_to_what_is_ready_and_counts_it() {
    let mut set = [0b0010_0110u8, 0, 0, 0, 0, 0, 0, 0];
    let kept = narrow(&mut set, 64, |fd| if fd == 2 { 1 } else { 0 }, 1);
    assert_eq!(kept, 1);
    assert_eq!(set[0], 0b0000_0100);
    // A set shorter than nfds is narrowed as far as it goes, never past it.
    let mut short = [0xffu8; 2];
    assert_eq!(narrow(&mut short, 256, |_| 1, 1), 16);
    assert!(is_set(&short, 15) && !is_set(&short, 16));
}

#[test]
fn an_epoll_entry_reports_level_and_edge_as_linux_does() {
    assert_eq!(live(IN | OUT, IN, 0), IN, "only what was asked for");
    assert_eq!(live(HUP, IN, 0), HUP, "hang-up is reported unasked");
    assert_eq!(live(IN, IN, IN), IN, "level-triggered: for as long as it holds");
    assert_eq!(live(IN, IN | EPOLLET, IN), 0, "edge-triggered: once per rise");
    assert_eq!(live(IN | OUT, IN | OUT | EPOLLET, IN), OUT, "a new edge is reported");
    assert_eq!(seen(IN | OUT | HUP, OUT), OUT | HUP);
}

#[test]
fn epollexclusive_is_allowed_only_where_linux_allows_it() {
    let table: [ExclusiveRow; 8] = [
        (ADD, EXCLUSIVE | IN | EPOLLET, false, None, Ok(())),
        (ADD, EXCLUSIVE | IN | EPOLLONESHOT, false, None, Err(EINVAL)),
        (ADD, EXCLUSIVE | IN, true, None, Err(EINVAL)),
        (MOD, EXCLUSIVE | IN, false, Some(IN), Err(EINVAL)),
        (MOD, IN, false, Some(EXCLUSIVE | IN), Err(EINVAL)),
        (MOD, IN | OUT, false, Some(IN), Ok(())),
        (ADD, IN | EPOLLONESHOT, true, None, Ok(())),
        (DEL, EXCLUSIVE, false, Some(EXCLUSIVE), Ok(())),
    ];
    for (op, events, target_epoll, present, want) in table {
        assert_eq!(exclusive_ok(op, events, target_epoll, present), want, "op {op} {events:#x}");
    }
}

/// Epolls 10.. as a table: each entry is the descriptors one watches.
fn graph<'a>(edges: &'a [(u64, &'a [u64])]) -> impl Fn(u64) -> Option<Vec<u64>> + 'a {
    move |fd| edges.iter().find(|(e, _)| *e == fd).map(|(_, w)| w.to_vec())
}

#[test]
fn an_epoll_may_not_watch_itself_through_others_or_nest_too_deep() {
    // 11 watches 12, which watches 10: adding 11 to 10 closes a loop.
    let lists = graph(&[(10, &[3]), (11, &[12]), (12, &[10, 4])]);
    assert!(loops(&lists, 10, 11));
    // A diamond is no loop.
    let lists = graph(&[(10, &[]), (11, &[12, 13]), (12, &[14]), (13, &[14]), (14, &[5])]);
    assert!(!loops(&lists, 10, 11));
    // A chain of MAX_NESTS epolls below the target is one too many.
    let deep: Vec<(u64, Vec<u64>)> =
        (0..=MAX_NESTS as u64 + 1).map(|i| (11 + i, vec![12 + i])).collect();
    let table: Vec<(u64, &[u64])> = deep.iter().map(|(e, w)| (*e, w.as_slice())).collect();
    assert!(loops(&graph(&table), 10, 11));
    let table: Vec<(u64, &[u64])> = table.into_iter().take(MAX_NESTS as usize).collect();
    assert!(!loops(&graph(&table), 10, 11));
}

#[test]
fn a_wide_web_of_epolls_is_walked_once_each() {
    use core::cell::Cell;
    // Five layers of fifty epolls, each watching every epoll of the next.
    let looks = Cell::new(0u32);
    let lists = |fd: u64| {
        looks.set(looks.get() + 1);
        let (layer, _) = ((fd - 100) / 50, (fd - 100) % 50);
        (100..350).contains(&fd).then(|| match layer {
            4 => vec![],
            l => (0..50).map(|i| 100 + (l + 1) * 50 + i).collect(),
        })
    };
    assert!(!loops(&lists, 99, 100));
    assert!(looks.get() < 250 * 60, "{} looks", looks.get());
}

#[test]
fn random_sets_and_counts_never_panic_and_narrowing_only_clears() {
    let mut r = Regs::new(0x7265_6164_795f_7365);
    for _ in 0..50_000 {
        let nfds_raw = r.arg();
        if let Ok(n) = select_count(nfds_raw, MOST) {
            assert!(n <= MOST);
        }
        if let Ok(n) = poll_count(nfds_raw, MOST) {
            assert!(n <= MOST);
        }
        let len = r.small(40) as usize;
        let before: Vec<u8> = (0..len).map(|_| r.any() as u8).collect();
        let mut set = before.clone();
        let nfds = r.small(400);
        let mask = r.any();
        let kept = narrow(&mut set, nfds, |fd| ((mask >> (fd % 64)) & 1) as u16, 1);
        let ones: u32 = set.iter().map(|b| b.count_ones()).sum();
        assert!(kept as u32 <= ones && set.iter().zip(&before).all(|(a, b)| a & !b == 0));
        let _ = first_closed(&[&before, &set], nfds, |fd| fd % 3 != 0);
        let (level, events, fired) = (r.any() as u32, r.any() as u32, r.any() as u32);
        assert_eq!(live(level, events, fired) & !seen(level, events), 0);
    }
}
