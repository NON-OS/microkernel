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

//! A look that ran out asks the launch again: busy while the core's run goes
//! on, the one Linux run there is, so a slow first start is waited for and
//! not taken for a core with no profile; 0 once it has ended, so the name is
//! asked of a core that had none and a named core is started again, at most
//! MAX_STARTS times. A run that never listens is stopped on after WAIT_MS.

use crate::model::start::{Phase, Start, DID_NOT_START, EBUSY, EVERY_MS, LOOK_MS, WAIT_MS};

/// The kernel's one run and a core in it: a launch is answered busy while a
/// run goes on, else starts one. `life` gives a start's word what that run
/// does, from its start: how long until it ends, and until it listens.
struct Kernel<F: Fn(Option<&str>) -> (i64, Option<i64>)> {
    life: F,
    run: Option<(i64, Option<i64>)>,
    words: Vec<Option<String>>,
}

impl<F: Fn(Option<&str>) -> (i64, Option<i64>)> Kernel<F> {
    fn new(life: F) -> Self {
        Kernel { life, run: None, words: Vec::new() }
    }

    fn launch(&mut self, word: Option<&str>, now: i64) -> i64 {
        if self.run.is_some_and(|(ends, _)| now < ends) {
            return EBUSY;
        }
        let (ends, listens) = (self.life)(word);
        self.run = Some((now + ends, listens.map(|l| now + l)));
        self.words.push(word.map(String::from));
        0
    }

    fn listening(&self, now: i64) -> bool {
        self.run.is_some_and(|(ends, listens)| now < ends && listens.is_some_and(|l| now >= l))
    }
}

/// The window against `k` for up to `until` ms, `name` given when asked.
/// The phase it ends in, and when.
fn drive<F: Fn(Option<&str>) -> (i64, Option<i64>)>(
    k: &mut Kernel<F>,
    name: &str,
    until: i64,
) -> (Phase, i64) {
    let mut s = Start::new();
    let mut asked = false;
    for now in (0..until).step_by(EVERY_MS as usize) {
        let step = s.tick(now);
        if let Some(word) = step.launch {
            let rc = k.launch(word.as_deref(), now);
            s.launched(rc, now);
        }
        if step.look && k.listening(now) {
            s.found();
        }
        if s.phase == Phase::Ask {
            assert!(!asked, "the name is asked once");
            asked = true;
            assert!(s.named(name));
        }
        if matches!(s.phase, Phase::Up | Phase::Stopped(_)) {
            return (s.phase, now);
        }
    }
    (s.phase, until)
}

const FOREVER: i64 = i64::MAX / 4;

#[test]
fn a_core_with_its_profile_that_listens_after_minutes_is_not_asked_a_name() {
    // An emulated machine: the core opens its databases for 3 minutes.
    let mut k = Kernel::new(|_| (FOREVER, Some(180_000)));
    let (phase, at) = drive(&mut k, "Ada", 900_000);
    assert_eq!(phase, Phase::Up);
    assert!(at >= 180_000);
    assert_eq!(k.words, [None], "one start, with no word: it already had its profile");
}

#[test]
fn a_named_first_start_that_migrates_for_minutes_comes_up() {
    // No profile: a start with no word ends at once; with the name it makes
    // the profile and migrates its two databases for 4 minutes.
    let mut k = Kernel::new(|w| match w {
        None => (1_000, None),
        Some(_) => (FOREVER, Some(240_000)),
    });
    let (phase, _) = drive(&mut k, "Ada", 900_000);
    assert_eq!(phase, Phase::Up, "a slow first start is waited for, not stopped on at LOOK_MS");
    let named = Some(String::from("--user-display-name=Ada"));
    assert_eq!(k.words.last(), Some(&named));
    assert_eq!(k.words.iter().filter(|w| **w == named).count(), 1, "started once with the name");
}

#[test]
fn a_core_with_no_profile_has_its_name_asked_once_its_run_ended() {
    let mut k = Kernel::new(|w| match w {
        None => (2_000, None),
        Some(_) => (FOREVER, Some(5_000)),
    });
    let (phase, at) = drive(&mut k, "Ada", 900_000);
    assert_eq!(phase, Phase::Up);
    assert!(at < 2 * LOOK_MS, "asked after one look, up {at} ms in");
}

#[test]
fn a_named_core_that_ends_each_time_is_started_a_bounded_number_of_times() {
    let mut k = Kernel::new(|_| (2_000, None));
    let (phase, at) = drive(&mut k, "Ada", 900_000);
    assert_eq!(phase, Phase::Stopped(DID_NOT_START));
    let named = k.words.iter().filter(|w| w.is_some()).count();
    assert_eq!(named, crate::model::start::MAX_STARTS as usize);
    assert!(at < 10 * LOOK_MS);
}

#[test]
fn a_run_that_never_listens_is_stopped_on_after_wait_ms() {
    let mut k = Kernel::new(|_| (FOREVER, None));
    let (phase, at) = drive(&mut k, "Ada", 2 * WAIT_MS);
    assert_eq!(phase, Phase::Stopped(DID_NOT_START));
    assert!((WAIT_MS..WAIT_MS + LOOK_MS).contains(&at), "stopped at {at} ms");
    assert_eq!(k.words, [None], "a run going on is never started twice");
}

#[test]
fn a_probe_asks_with_the_word_its_run_was_started_with() {
    let mut s = Start::new();
    let (_, at) = crate::start_tests::launch_at(&mut s, 0, 0);
    let (_, at) = crate::start_tests::launch_at(&mut s, at + LOOK_MS, 0);
    assert!(s.named("Ada"));
    let (_, at) = crate::start_tests::launch_at(&mut s, at + EVERY_MS, 0);
    s.found();
    s.lost(at + 10);
    let (w, at) = crate::start_tests::launch_at(&mut s, at + 10 + EVERY_MS, 0);
    assert_eq!(w, None, "a start after a loss has no word: the profile may be renamed");
    let (w, _) = crate::start_tests::launch_at(&mut s, at + LOOK_MS, EBUSY);
    assert_eq!(w, None, "its probe has none either");
    assert!(matches!(s.phase, Phase::Launch(None)));
}
