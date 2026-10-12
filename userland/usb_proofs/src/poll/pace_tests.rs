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

//! The poll loop asked every bound endpoint for a report on every pass of a
//! 1 ms inbox wait: on an idle machine a thousand passes and IPC round trips
//! a second per endpoint. It now asks each as often as the controller
//! serves it, every 8 ms once quiet, and sleeps on its inbox in between.

use super::pace::{inbox_wait_ms, Pace, MAX_WAIT_MS, QUIET_PERIOD_MS};

/// The loop over `ms` from `from`, endpoint i having a report at `now` when
/// `has(i, now)`: the times each was asked, and the loop's passes.
fn run(paces: &mut [Pace], from: u64, ms: u64, has: impl Fn(usize, u64) -> bool) -> Vec<Vec<u64>> {
    let (mut now, mut asks) = (from, vec![Vec::new(); paces.len() + 1]);
    while now < from + ms {
        for (i, p) in paces.iter_mut().enumerate().filter(|(_, p)| p.due(now)) {
            asks[i].push(now);
            p.asked(now, has(i, now));
        }
        asks[paces.len()].push(now);
        now += inbox_wait_ms(now, paces.iter());
    }
    asks
}

#[test]
fn an_idle_keyboard_and_mouse_are_asked_every_8_ms_not_every_1() {
    // A full speed keyboard (8 ms) and a 1000 Hz mouse, ten quiet seconds.
    let mut paces = [Pace::new(8), Pace::new(1)];
    let asks = run(&mut paces, 5_000, 10_000, |_, _| false);
    let most = 10_000 / QUIET_PERIOD_MS as usize + 1;
    for (who, a) in ["keyboard", "mouse", "loop passes"].iter().zip(&asks) {
        assert!(a.len() <= most, "{who}: {} in ten seconds", a.len());
    }
}

#[test]
fn a_streaming_mouse_is_asked_every_ms_and_a_quiet_one_answered_within_8() {
    let mut paces = [Pace::new(8), Pace::new(1)];
    let asks = run(&mut paces, 5_000, 1_000, |i, _| i == 1);
    assert_eq!(asks[1].len(), 1_000, "every millisecond");
    assert!(asks[0].len() <= 126, "the quiet keyboard: {}", asks[0].len());
    // Quiet for a second, then moved at 7_003.
    let asks = run(&mut paces, 6_000, 2_000, |i, now| i == 1 && now >= 7_003);
    let first = asks[1].iter().find(|&&t| t >= 7_003).copied().unwrap();
    assert!(first - 7_003 < QUIET_PERIOD_MS, "heard {} ms late", first - 7_003);
    assert!(asks[1].windows(2).filter(|w| w[0] > 7_003).all(|w| w[1] - w[0] == 1));
}

#[test]
fn the_inbox_wait_is_until_the_next_ask_and_never_0_or_past_16_ms() {
    assert_eq!(inbox_wait_ms(100, [].iter()), MAX_WAIT_MS, "nothing bound");
    let mut p = Pace::new(5);
    assert_eq!(inbox_wait_ms(100, [p].iter()), 1, "due now; 0 would wait for good");
    p.asked(100, true);
    assert_eq!(inbox_wait_ms(101, [p].iter()), 4);
    let q = Pace::new(4096);
    let mut slow = q;
    slow.asked(100, true);
    assert_eq!(inbox_wait_ms(101, [slow].iter()), MAX_WAIT_MS);
    assert_eq!(inbox_wait_ms(101, [slow, p].iter()), 4, "the first due wins");
}
