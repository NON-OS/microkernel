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

use super::served_rule::{step, wait_ms, Step, Tries, FIRST_WAIT_MS, LONGEST_WAIT_MS};
use super::stage::Stage;

const SERVED: &str = include_str!("../../../../src/userspace/init/linux_jobs/served.rs");
const QUEUE: &str = include_str!("../../../../src/userspace/init/linux_jobs/queue.rs");

/// What the launches do, one a millisecond from `from` to `to`, with the
/// stage the listing holds the whole time: how many installs they queue.
fn installs(stage: Stage, kept: &mut Option<Tries>, from: u64, to: u64) -> usize {
    let mut n = 0;
    for now in from..to {
        let (s, k) = step(Some(stage), *kept, now);
        n += usize::from(s == Step::Install);
        *kept = k;
    }
    n
}

#[test]
fn a_core_not_installed_is_installed_and_a_placed_one_run() {
    let (s, k) = step(None, None, 5);
    assert_eq!(s, Step::Install);
    assert_eq!(k, Some(Tries { failed: 0, at_ms: 5, queued: true }));
    assert_eq!(step(Some(Stage::Removed), None, 5).0, Step::Install);
    assert_eq!(step(Some(Stage::Installed), k, 9), (Step::Run, None));
    for moving in [Stage::Queued, Stage::Running(4)] {
        assert_eq!(step(Some(moving), k, 9), (Step::Wait, k));
    }
}

#[test]
fn a_failure_no_served_launch_queued_is_installed_again_at_once() {
    for failed in [Stage::Failed(3), Stage::Refused] {
        assert_eq!(step(Some(failed), None, 7).0, Step::Install);
    }
}

#[test]
fn an_install_it_queued_that_failed_waits_before_the_next() {
    let mut kept = step(None, None, 0).1;
    // The install fails at 1000; the window asks every millisecond after.
    let first_wait = 1000 + FIRST_WAIT_MS;
    assert_eq!(installs(Stage::Failed(5), &mut kept, 1000, first_wait), 0);
    assert_eq!(kept, Some(Tries { failed: 1, at_ms: 1000, queued: false }));
    assert_eq!(installs(Stage::Failed(5), &mut kept, first_wait, first_wait + 1), 1);
    assert_eq!(kept, Some(Tries { failed: 1, at_ms: 1000, queued: true }));
}

#[test]
fn each_failure_in_a_row_waits_twice_as_long_up_to_the_longest() {
    assert_eq!(wait_ms(0), 0);
    assert_eq!(wait_ms(1), FIRST_WAIT_MS);
    assert_eq!(wait_ms(2), 2 * FIRST_WAIT_MS);
    assert_eq!(wait_ms(3), 4 * FIRST_WAIT_MS);
    assert_eq!(wait_ms(5), LONGEST_WAIT_MS);
    for failed in [6, 17, 64, u32::MAX] {
        assert_eq!(wait_ms(failed), LONGEST_WAIT_MS, "{failed}");
    }
}

#[test]
fn an_hour_of_outage_queues_a_bounded_number_of_installs() {
    // Every install fails the moment it is queued; asked every 10 ms.
    let mut kept = None;
    let mut queued = 0usize;
    let mut stage = None;
    let hour = 3_600_000u64;
    for now in (0..hour).step_by(10) {
        let (s, k) = step(stage, kept, now);
        kept = k;
        if s == Step::Install {
            queued += 1;
            stage = Some(Stage::Failed(5));
        }
    }
    // 15 + 30 + 60 + 120 s, then one every 240 s: far under a tick's worth.
    let bound = 5 + usize::try_from(hour / LONGEST_WAIT_MS).unwrap();
    assert!(queued <= bound, "{queued} installs in an hour");
    assert!(queued >= 4, "{queued}: it must still try again");
}

#[test]
fn success_after_failures_forgets_them() {
    let kept = Some(Tries { failed: 4, at_ms: 0, queued: true });
    assert_eq!(step(Some(Stage::Installed), kept, 1), (Step::Run, None));
    // A later loss (the core uninstalled) installs at once.
    assert_eq!(step(Some(Stage::Removed), None, 2).0, Step::Install);
}

#[test]
fn a_clock_read_earlier_than_the_failure_waits() {
    let kept = Some(Tries { failed: 1, at_ms: 50_000, queued: false });
    assert_eq!(step(Some(Stage::Failed(1)), kept, 10), (Step::Wait, kept));
}

#[test]
fn the_kernel_keeps_tries_only_for_what_it_queued() {
    // A refused install is not one that failed later.
    assert!(SERVED.contains("if done {"));
    assert!(SERVED.contains(
        "Step::Install => super::queue::request_install(listing.clone(), String::new()),"
    ));
    assert!(QUEUE.contains("super::served::request(listing, package)"));
}

#[test]
fn a_served_launch_is_answered_busy_until_the_cores_run_is_queued() {
    // The window looks for its core 20 s after a launch answered 0, then
    // stops: an install still moving must be answered busy, so it asks again.
    let at = SERVED.find("pub(super) fn request(").expect("request");
    let body = &SERVED[at..at + SERVED[at..].find("\n}\n").expect("end")];
    assert!(body.trim_end().ends_with("done && next == Step::Run"), "{body}");
    let launch = include_str!("../../../../src/syscall/microkernel/app_launch.rs");
    assert!(launch
        .contains("Launch::Served => crate::userspace::init::request_served_run(listing, name),"));
    let busy = launch.find("false => {").expect("a false answer");
    assert!(launch[busy..].contains("ERRNO_BUSY"));
    let window = include_str!("../../../../userland/capsule_simplex_ui/src/model/start.rs");
    assert!(window.contains("EBUSY => return,"), "the window asks again on busy");
}
