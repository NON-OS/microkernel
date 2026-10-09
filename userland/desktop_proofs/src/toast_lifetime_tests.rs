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

//! No notice stays on screen. "Terminal opened", "Files closed" and the rest
//! leave `TOAST_LIFETIME_MS` after they were first shown: with nothing else
//! happening, after the app that caused them ended, when they are said again
//! and again, and when the clock reads behind the time they were shown. The
//! serve loop is modelled as `server/runner/run.rs` and `drain.rs` run it: it
//! waits on its inbox until `wake_at`, expires the queue on every pass, and
//! syncs the panel whenever the queue moved.

use crate::shell_tick::{tick_due, wait_ms, wake_at, wake_due, TICK_MS};
use crate::toast_state::toast::{UptimeMs, TOAST_HELD_MS, TOAST_LIFETIME_MS};
use crate::toast_state::toasts::ToastQueue;
use crate::toast_state::NotifyLevel;

/// The serve loop's longest wait on an idle inbox (runner/constants.rs).
const RECV_BLOCK: u64 = 1000;

/// What the screen shows: the queue as of the last panel sync.
struct Screen {
    synced: u32,
    shown: usize,
}

/// The serve loop with an inbox that stays empty: no pointer, no app, no
/// other repaint. Runs from uptime `now` until `until`, its tick running on
/// whole seconds of uptime as the shell's does from boot, and gives back the
/// uptime at which the panel was last seen to empty, if it did.
fn idle_loop(q: &mut ToastQueue, s: &mut Screen, mut now: i64, until: i64) -> Option<i64> {
    let mut last_tick = now - now.rem_euclid(TICK_MS);
    let mut emptied = None;
    while now < until {
        let wake = wake_at(last_tick, q.next_expiry().map(|t| t.0));
        now += wait_ms(now, wake, RECV_BLOCK) as i64;
        if tick_due(now, last_tick) {
            last_tick = now;
        }
        q.expire(UptimeMs(now));
        if s.synced != q.generation() {
            s.synced = q.generation();
            let was = s.shown;
            s.shown = q.iter_live().count();
            if was != 0 && s.shown == 0 {
                emptied = Some(now);
            }
        }
    }
    emptied
}

#[test]
fn the_lifetime_is_short() {
    const { assert!(TOAST_LIFETIME_MS >= 2000 && TOAST_LIFETIME_MS <= 3000) };
}

#[test]
fn an_expired_toast_is_no_longer_painted_with_no_other_input() {
    let mut q = ToastQueue::new();
    let mut s = Screen { synced: q.generation(), shown: 0 };
    // A launch at uptime 1234, off any tick boundary.
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(1234));
    s.synced = q.generation();
    s.shown = 1;
    let gone = idle_loop(&mut q, &mut s, 1234, 1234 + 10_000);
    assert_eq!(
        gone,
        Some(1234 + TOAST_LIFETIME_MS),
        "the loop wakes for the expiry itself, not on the next tick or repaint"
    );
    assert_eq!(s.shown, 0);
}

#[test]
fn every_transient_notice_leaves_within_three_seconds() {
    let words: [(&[u8], NotifyLevel); 8] = [
        (b"Terminal opened", NotifyLevel::Info),
        (b"Terminal closed", NotifyLevel::Info),
        (b"opening a new window", NotifyLevel::Info),
        (b"window limit reached, focusing", NotifyLevel::Warn),
        (b"Files did not open", NotifyLevel::Error),
        (b"network connected", NotifyLevel::Info),
        (b"saved /home/notes.txt", NotifyLevel::Info),
        (b"clipboard unavailable", NotifyLevel::Warn),
    ];
    for (i, (text, level)) in words.into_iter().enumerate() {
        let mut q = ToastQueue::new();
        let mut s = Screen { synced: q.generation(), shown: 0 };
        let at = 500 + 137 * i as i64;
        q.push(text, level, UptimeMs(at));
        s.synced = q.generation();
        s.shown = 1;
        let gone =
            idle_loop(&mut q, &mut s, at, at + 10_000).expect("every notice leaves on its own");
        assert!(gone - at <= 3000, "a notice stayed {} ms", gone - at);
    }
}

#[test]
fn a_notice_whose_app_ended_is_never_cleared_and_still_leaves() {
    // The app opened, its notice went up, and the app is gone: nothing ever
    // clears the notice or says anything more about that app.
    let mut q = ToastQueue::new();
    let mut s = Screen { synced: q.generation(), shown: 0 };
    q.push(b"Calculator opened", NotifyLevel::Info, UptimeMs(0));
    q.push(b"Calculator closed", NotifyLevel::Info, UptimeMs(300));
    s.synced = q.generation();
    s.shown = 2;
    let gone = idle_loop(&mut q, &mut s, 300, 60_000);
    assert_eq!(gone, Some(300 + TOAST_LIFETIME_MS));
    assert!(q.is_empty());
}

#[test]
fn a_repeated_notice_keeps_the_expiry_of_its_first_showing() {
    let mut q = ToastQueue::new();
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(0));
    let first = q.next_expiry();
    let generation = q.generation();
    // Said again every 100 ms while it is up.
    let mut now = 100;
    while now < TOAST_LIFETIME_MS {
        q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(now));
        assert_eq!(q.next_expiry(), first, "a repeat does not re-arm the timer");
        assert_eq!(q.iter_live().count(), 1, "a repeat is not a second row");
        now += 100;
    }
    assert_eq!(q.generation(), generation, "a repeat changes nothing on screen");
    assert!(q.expire(UptimeMs(TOAST_LIFETIME_MS)), "it leaves when its first showing is up");
    assert!(q.is_empty());
}

#[test]
fn a_notice_said_without_pause_never_outlives_its_lifetime() {
    // Quick open and close of the same app, said every 50 ms for a minute:
    // whatever is up at any moment was first shown less than a lifetime ago.
    let mut q = ToastQueue::new();
    let mut now = 0;
    while now < 60_000 {
        let text: &[u8] = if (now / 50) % 2 == 0 { b"Files opened" } else { b"Files closed" };
        q.push(text, NotifyLevel::Info, UptimeMs(now));
        q.expire(UptimeMs(now));
        for t in q.iter_live() {
            assert!(now - t.shown_at_ms < TOAST_LIFETIME_MS, "a notice outlived its lifetime");
        }
        now += 50;
    }
}

#[test]
fn a_clock_reading_behind_the_showing_does_not_hold_a_notice() {
    // The wall clock steps back when NTP corrects an RTC kept in local time;
    // a toast stamped before the step stayed up for the whole step. The
    // queue takes uptime only, and a reading behind a toast ends it.
    let mut q = ToastQueue::new();
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(7_200_000));
    assert!(q.expire(UptimeMs(5_000)), "a clock that went back does not hold it up");
    assert!(q.is_empty());
}

#[test]
fn a_clock_that_never_advances_is_not_the_one_toasts_use() {
    // The wall clock reads as the same error code (-61) on every call until
    // the RTC is read; a toast timed on it was never let go. On uptime, which
    // the loop's own wait advances, it leaves.
    let mut q = ToastQueue::new();
    let mut s = Screen { synced: q.generation(), shown: 0 };
    q.push(b"Terminal opened", NotifyLevel::Info, UptimeMs(0));
    s.synced = q.generation();
    s.shown = 1;
    assert!(idle_loop(&mut q, &mut s, 0, 5_000).is_some());
}

#[test]
fn a_stream_of_messages_does_not_hold_the_loop_in_the_drain() {
    // A pointer moving, or an app redrawing, sends faster than any wait; the
    // drain still goes back to the loop once its wake is due.
    let mut q = ToastQueue::new();
    q.push(b"Files closed", NotifyLevel::Info, UptimeMs(0));
    let wake = wake_at(0, q.next_expiry().map(|t| t.0));
    let mut now = 0;
    let mut handled = 0;
    while !wake_due(now, wake) {
        now += 4;
        handled += 1;
        assert!(handled < 100_000, "the drain never gave the loop back");
    }
    assert!(now <= wake + 4);
}

#[test]
fn the_wake_is_never_later_than_the_tick_or_the_expiry() {
    assert_eq!(wake_at(0, None), TICK_MS);
    assert_eq!(wake_at(0, Some(300)), 300);
    assert_eq!(wake_at(0, Some(5000)), TICK_MS);
    assert_eq!(wait_ms(0, 300, RECV_BLOCK), 300);
    assert_eq!(wait_ms(0, 5000, RECV_BLOCK), RECV_BLOCK);
    assert_eq!(wait_ms(400, 300, RECV_BLOCK), 1, "a wake already due waits as little as it can");
}

#[test]
fn the_store_warning_is_held_by_design_bounded_and_dismissed_by_a_press() {
    let mut q = ToastQueue::new();
    q.push_held(b"The capsule store could not be read", NotifyLevel::Error, UptimeMs(0));
    assert!(!q.expire(UptimeMs(TOAST_HELD_MS - 1)), "held up longer than a transient notice");
    assert!(q.expire(UptimeMs(TOAST_HELD_MS)), "but not for ever");

    let mut q = ToastQueue::new();
    let g = q.generation();
    q.push_held(b"The capsule store could not be read", NotifyLevel::Error, UptimeMs(0));
    q.clear();
    assert!(q.is_empty(), "a press on the panel dismisses it");
    assert_ne!(q.generation(), g);
}
