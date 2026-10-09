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

//! A core service that ends is started again, spaced out and a bounded
//! number of times. Before, nothing called the supervisor, so one crash in
//! net.sockets, vfs or the compositor lasted until a reboot.

use crate::lifecycle_state::CapsuleState;
use crate::watch_rule::{is_watched, next, Next, SETTLE_MS, WATCHED};

#[test]
fn a_running_or_never_started_service_is_left_alone() {
    for (due, spent) in [(false, false), (true, false), (false, true), (true, true)] {
        for dead_for in [0, SETTLE_MS, u64::MAX] {
            assert_eq!(next(true, true, due, spent, dead_for), Next::Leave, "running");
            assert_eq!(next(false, false, due, spent, dead_for), Next::Leave, "never started");
        }
    }
}

#[test]
fn an_ended_service_is_restarted_when_due_and_given_up_on_once_spent() {
    assert_eq!(next(false, true, true, false, SETTLE_MS), Next::Restart);
    assert_eq!(next(false, true, false, false, SETTLE_MS), Next::Leave, "not due yet");
    assert_eq!(next(false, true, true, true, SETTLE_MS), Next::GiveUp);
    assert_eq!(next(false, true, false, true, 0), Next::GiveUp);
}

/*
 * The exit teardown drops a dead service's endpoints on a later timer tick.
 * A restart before that finds its name still registered, and the spawn fails
 * after its process was made.
 */
#[test]
fn a_service_is_restarted_only_after_its_exit_has_settled() {
    assert_eq!(next(false, true, true, false, 0), Next::Leave, "just seen ended");
    assert_eq!(next(false, true, true, false, SETTLE_MS - 1), Next::Leave);
    assert_eq!(next(false, true, true, false, SETTLE_MS), Next::Restart);
}

#[test]
fn restarts_are_spaced_out_and_end_at_the_limit() {
    let s = CapsuleState::new();
    let t = 50_000u64;
    assert!(s.should_respawn(t), "the first end is restarted at once");
    s.record_exit(t);
    assert!(!s.should_respawn(t + 1_999), "two seconds between restarts");
    assert!(s.should_respawn(t + 2_000));
    let mut now = t;
    while s.restart_count() < s.max_restarts() {
        now += 2_000;
        s.record_exit(now);
    }
    assert_eq!(s.restart_count(), 8);
    assert!(!s.should_respawn(now + 1_000_000), "no restart past the limit, ever");
}

#[test]
fn no_driver_app_or_one_shot_capsule_is_watched() {
    const ONE_SHOT: [&str; 7] = [
        "setup_wizard",
        "installer",
        "login",
        "boot_splash",
        "input_probe",
        "toolkit",
        "app_linux",
    ];
    for name in WATCHED {
        assert!(!name.starts_with("driver_"), "{name}: a driver exits on purpose when absent");
        assert!(!name.starts_with("app_"), "{name}: an app ends when the person closes it");
        assert!(!ONE_SHOT.contains(name), "{name} ends on purpose");
    }
    assert!(!is_watched("driver_nvme") && is_watched("net_sockets") && is_watched("compositor"));
}

/*
 * The list names capsules by the lifecycle name the spawn plan registers. A
 * name that matches none would leave that service unwatched without a word.
 */
#[test]
fn every_watched_name_is_one_the_spawn_plan_registers() {
    /* Every place init boots a capsule by name: the spawn plan, and init's
     * own entry (shield). The supervisor's list itself is left out, or every
     * name would be found in it. */
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/userspace/init");
    let mut text = String::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("readable").flatten() {
            let path = entry.path();
            if path.is_dir() && !path.ends_with("supervisor") {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                text.push_str(&std::fs::read_to_string(&path).expect("readable"));
            }
        }
    }
    // The shield boots from init's entry, after the network it reads over.
    let entry = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../src/userspace/init/entry.rs");
    text.push_str(&std::fs::read_to_string(entry).expect("readable"));
    for name in WATCHED {
        assert!(text.contains(&format!("\"{name}\"")), "{name} is not a name init boots a capsule by");
    }
}
