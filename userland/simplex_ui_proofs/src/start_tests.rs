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

//! Starting the core: no word first, the name asked for only when no core
//! listened in LOOK_MS and its run had ended, a busy launch asked again, and
//! the endpoint used whenever it answers.

use crate::model::start::{Phase, Start, EBUSY, EVERY_MS, LOOK_MS};

/// Ticks from `from` until a launch is asked for, answering each with `rc`.
pub fn launch_at(s: &mut Start, from: i64, rc: i64) -> (Option<String>, i64) {
    let mut now = from;
    loop {
        if let Some(w) = s.tick(now).launch {
            s.launched(rc, now);
            return (w, now);
        }
        now += EVERY_MS;
    }
}

#[test]
fn the_first_launch_has_no_word_and_a_core_that_answers_is_used() {
    let mut s = Start::new();
    let (w, at) = launch_at(&mut s, 0, 0);
    assert_eq!(w, None);
    assert!(s.tick(at + EVERY_MS).look);
    s.found();
    assert_eq!(s.phase, Phase::Up);
    let step = s.tick(at + 2 * EVERY_MS);
    assert!(step.launch.is_none() && !step.look);
}

#[test]
fn a_core_that_never_listens_has_its_name_asked_after_look_ms() {
    let mut s = Start::new();
    let (_, at) = launch_at(&mut s, 0, 0);
    assert!(s.tick(at + LOOK_MS - EVERY_MS).look);
    assert!(matches!(s.phase, Phase::Look { .. }));
    let step = s.tick(at + LOOK_MS);
    assert!(step.look, "it still looks while it asks");
    assert_eq!(step.launch, Some(None), "the look ran out: is the run still going?");
    s.launched(0, at + LOOK_MS);
    assert_eq!(s.phase, Phase::Ask, "it had ended: a core with no profile");
    assert!(s.named("  Ada Lovelace "));
    let (w, _) = launch_at(&mut s, at + LOOK_MS + EVERY_MS, 0);
    assert_eq!(w.as_deref(), Some("--user-display-name=Ada Lovelace"));
}

#[test]
fn a_busy_launch_is_asked_again_and_looked_for_meanwhile() {
    let mut s = Start::new();
    let (_, at) = launch_at(&mut s, 0, EBUSY);
    assert_eq!(s.phase, Phase::Launch(None));
    let step = s.tick(at + EVERY_MS);
    assert!(step.launch.is_some() && step.look);
    s.found();
    assert_eq!(s.phase, Phase::Up, "an endpoint that answers while busy is used");
}

/// The window against the kernel's served launch, the core's install
/// taking `install_ms` and the core listening `listen_ms` after its run:
/// the kernel answers busy (`busy` true) or 0 (`busy` false) while the
/// install moves, then 0 with the run queued. The phase it ends in.
fn first_start(install_ms: i64, listen_ms: i64, busy: bool) -> Phase {
    let mut s = Start::new();
    let mut ran = None;
    for now in (0..600_000).step_by(EVERY_MS as usize) {
        let step = s.tick(now);
        if let Some(_word) = step.launch {
            let placed = now >= install_ms;
            let rc = if placed || !busy { 0 } else { EBUSY };
            if placed && ran.is_none() {
                ran = Some(now);
            }
            s.launched(rc, now);
        }
        if step.look && ran.is_some_and(|at| now >= at + listen_ms) {
            s.found();
        }
        if s.phase == Phase::Ask {
            assert!(s.named("Ada"), "the person gives a name when asked");
        }
        if matches!(s.phase, Phase::Up | Phase::Stopped(_)) {
            break;
        }
    }
    s.phase
}

#[test]
fn a_core_that_installs_for_minutes_still_comes_up() {
    // The kernel answers a served launch busy until the core's run is
    // queued (init linux_jobs served.rs), and the window asks again.
    for install_ms in [0, 5_000, 30_000, 90_000, 300_000] {
        assert_eq!(first_start(install_ms, 2_000, true), Phase::Up, "{install_ms} ms");
    }
}

#[test]
fn answered_0_while_it_installs_the_window_gave_up() {
    // What the window did when the kernel answered 0 for an install only
    // queued: it looked for a core that was not there, then stopped.
    assert!(matches!(first_start(90_000, 2_000, false), Phase::Stopped(_)));
}
