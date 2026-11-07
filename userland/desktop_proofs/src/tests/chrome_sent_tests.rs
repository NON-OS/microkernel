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
//! A frame the compositor took is damaged by the request that took it: a
//! present by the part it names, a whole submit by the whole screen. The
//! shell commits after a paint only the parts no such request damaged
//! (capsule_desktop_shell render/chrome/sent.rs). A commit of every part
//! after every paint composed the menu bar's strip twice each minute, and
//! the whole screen twice for each whole repaint.

use crate::chrome_sent::{sent, Owed, Painted, Sent};

const ALL: [Sent; 4] = [Sent::Same, Sent::Presented, Sent::Submitted, Sent::Unsaid];
const BAR: (u32, u32, u32, u32) = (0, 0, 1920, 58);
const DOCK: (u32, u32, u32, u32) = (360, 960, 1200, 120);
const TOASTS: (u32, u32, u32, u32) = (1480, 890, 420, 96);

fn asked(new: bool, flips: bool, answer: Sent) -> (Sent, bool) {
    let mut asked = false;
    let s = sent(new, flips, || {
        asked = true;
        answer
    });
    (s, asked)
}

#[test]
fn a_frame_the_same_as_on_screen_is_not_shown() {
    for flips in [false, true] {
        assert_eq!(asked(false, flips, Sent::Presented), (Sent::Same, false));
    }
}

#[test]
fn a_new_frame_on_one_backing_is_unsaid_and_never_flipped() {
    assert_eq!(asked(true, false, Sent::Presented), (Sent::Unsaid, false));
}

#[test]
fn a_new_frame_on_two_backings_is_what_its_flip_says() {
    for answer in [Sent::Presented, Sent::Submitted, Sent::Unsaid] {
        assert_eq!(asked(true, true, answer), (answer, true));
    }
    assert!(Sent::Presented.taken() && Sent::Submitted.taken());
    assert!(!Sent::Same.taken() && !Sent::Unsaid.taken(), "the shown backing stays shown");
}

/// The parts a model compositor still needs told after the frame was
/// shown: every part a changed frame put on screen, less what the requests
/// it took damaged (a present its first part, a submit all of them).
fn needed(p: Painted, parts: &[(u32, u32, u32, u32)]) -> Vec<(u32, u32, u32, u32)> {
    let mut damaged = vec![false; parts.len()];
    for s in [p.desk, p.chrome] {
        match s {
            Sent::Presented => damaged[0] = true,
            Sent::Submitted => damaged.iter_mut().for_each(|d| *d = true),
            Sent::Same | Sent::Unsaid => {}
        }
    }
    let unsaid = p.desk == Sent::Unsaid || p.chrome == Sent::Unsaid;
    let shown = p.changed();
    parts.iter().zip(&damaged).filter(|(_, &d)| shown && (unsaid || !d)).map(|(&a, _)| a).collect()
}

#[test]
fn every_part_on_screen_is_damaged_once_and_only_once() {
    for parts in [&[BAR][..], &[BAR, DOCK], &[BAR, DOCK, TOASTS]] {
        for desk in ALL {
            for chrome in ALL {
                let p = Painted { desk, chrome };
                let committed = p.owed().of(parts).to_vec();
                assert_eq!(committed, needed(p, parts), "{desk:?} {chrome:?} {}", parts.len());
            }
        }
    }
}

#[test]
fn a_taken_frame_commits_what_its_request_did_not_damage() {
    let painted = |desk, chrome| Painted { desk, chrome }.owed();
    assert_eq!(painted(Sent::Same, Sent::Presented), Owed::AfterFirst, "the clock, a menu row");
    assert_eq!(painted(Sent::Presented, Sent::Presented), Owed::AfterFirst);
    assert_eq!(painted(Sent::Submitted, Sent::Submitted), Owed::Nothing, "a whole repaint");
    assert_eq!(painted(Sent::Presented, Sent::Submitted), Owed::Nothing, "the screen is damaged");
    assert_eq!(painted(Sent::Same, Sent::Same), Owed::Nothing, "the frame on screen already");
    assert_eq!(painted(Sent::Submitted, Sent::Unsaid), Owed::Every, "the chrome drawn in place");
    assert_eq!(Owed::AfterFirst.of(&[BAR]), [] as [(u32, u32, u32, u32); 0]);
    assert_eq!(Owed::AfterFirst.of(&[BAR, DOCK, TOASTS]), [DOCK, TOASTS]);
    assert_eq!(Owed::Every.of(&[BAR, DOCK]), [BAR, DOCK]);
    assert!(Owed::Every.of::<u8>(&[]).is_empty());
}

#[test]
fn the_clock_minute_composes_the_bar_once() {
    // Presented with the bar named: the bar was asked for twice, by the
    // present and by the commit after it, and is asked for once.
    let p = Painted { desk: Sent::Same, chrome: Sent::Presented };
    let px =
        |a: &[(u32, u32, u32, u32)]| a.iter().map(|r| u64::from(r.2) * u64::from(r.3)).sum::<u64>();
    let before = px(&[BAR]) + px(&[BAR]);
    let after = px(&[BAR]) + px(p.owed().of(&[BAR]));
    assert_eq!((before, after), (2 * 111_360, 111_360));
    // A whole repaint: 2,073,600 pixels asked for once, not twice.
    let whole = (0, 0, 1920, 1080);
    let p = Painted { desk: Sent::Submitted, chrome: Sent::Submitted };
    assert_eq!(px(&[whole]) + px(p.owed().of(&[whole])), 2_073_600);
}

#[test]
fn every_caller_commits_only_what_is_owed() {
    const SHOW: &str = include_str!("../../../capsule_desktop_shell/src/render/chrome/show.rs");
    assert!(SHOW.contains("sent(desk, ctx.desk.flips(), || flip(ctx, ctx.desk, (DESK_Z, false), desk_at))"));
    assert!(SHOW.contains("if desk.taken() {\n        ctx.desk.flipped();"));
    assert!(SHOW.contains("if chrome.taken() {\n        ctx.chrome.flipped();"));
    const REPAINT: &str = include_str!("../../../capsule_desktop_shell/src/server/repaint.rs");
    assert!(REPAINT.contains("in paint_chrome(ctx).owed().of(&[whole]) {"));
    assert!(REPAINT.contains("in painted.owed().of(parts.as_slice()) {"));
    const CLOCK: &str =
        include_str!("../../../capsule_desktop_shell/src/server/runner/refresh_clock.rs");
    assert!(CLOCK.contains("in paint_chrome_in(ctx, changed).owed().of(&[r]) {"));
    const DOCK: &str = include_str!("../../../capsule_desktop_shell/src/server/refresh_taskbar.rs");
    assert!(DOCK.contains("in paint_chrome_in(ctx, Changed::Whole).owed().of(&dock) {"));
    const BAND: &str = include_str!("../../../capsule_desktop_shell/src/server/lock/band.rs");
    assert!(BAND.contains(".owed().of(&[r]) {"));
    for src in [REPAINT, CLOCK, DOCK, BAND] {
        let commits = src.matches("push_damage_commit(ctx.ports").count();
        let owed = src.matches(".owed().of(").count();
        assert_eq!(commits, owed, "every commit is of what is owed");
    }
}
