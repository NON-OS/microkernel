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

//! A wide web of epolls, and random sets and counts.

use super::super::super::random::Regs;
use super::select::MOST;
use crate::linux::file::epoll_rules::{live, loops, seen};
use crate::linux::net::ready_sets::{first_closed, narrow, poll_count, select_count};

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
