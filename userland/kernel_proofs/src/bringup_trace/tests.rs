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

use super::ev::{Ev, State};
use super::ledger::{Ledger, Tally, DEVICES};

const OFF: &str = include_str!("../../../../src/sys/bringup/off.rs");
const MOD: &str = include_str!("../../../../src/sys/bringup/mod.rs");
const CARGO: &str = include_str!("../../../../Cargo.toml");

#[test]
fn each_device_counts_by_its_last_event() {
    let mut l = Ledger::new();
    l.note("ACPI", &Ev::Spawned);
    l.note("DRIVERS", &Ev::Waiting("acpi-first-pass"));
    l.note("DRIVER-SDHCI", &Ev::Absent("no controller present"));
    l.note("BATTERY", &Ev::Absent("firmware reports none"));
    l.note("BATTERY", &Ev::Published);
    l.note("DRIVERS", &Ev::GaveUp("acpi-first-pass"));
    l.note("DRIVER-NVME", &Ev::Failed);
    let t = l.tally();
    let want = Tally { devices: 5, up: 2, absent: 1, waiting: 0, failed: 2, dropped: 0 };
    assert_eq!(t, want, "a battery that came later counts as up");
}

#[test]
fn a_device_past_the_table_is_counted_dropped() {
    const NAMES: [&str; 3] = ["A", "B", "C"];
    let mut l = Ledger::new();
    for i in 0..DEVICES {
        // Distinct names from one static string each: leak a few bytes.
        let name: &'static str = Box::leak(format!("D{i}").into_boxed_str());
        l.note(name, &Ev::Spawned);
    }
    for n in NAMES {
        l.note(n, &Ev::Spawned);
    }
    let t = l.tally();
    assert_eq!((t.devices as usize, t.up as usize, t.dropped), (DEVICES, DEVICES, 3));
}

#[test]
fn the_words_and_states_are_fixed() {
    let all = [
        (Ev::Spawned, "spawned", State::Up),
        (Ev::Failed, "failed", State::Failed),
        (Ev::Waiting("x"), "waiting", State::Waiting),
        (Ev::Ready("x"), "ready", State::Up),
        (Ev::GaveUp("x"), "gave-up", State::Failed),
        (Ev::Probed, "probed", State::Up),
        (Ev::Published, "published", State::Up),
        (Ev::Absent("y"), "absent", State::Absent),
    ];
    for (ev, word, state) in all {
        assert_eq!((ev.word(), ev.state()), (word.as_bytes(), state));
    }
    assert_eq!((Ev::Waiting("x").on(), Ev::Absent("y").why()), (Some("x"), Some("y")));
}

/// Off, each hook is an empty inline body: the boot path carries nothing.
#[test]
fn off_the_hooks_are_empty() {
    let flat: String = OFF.split_whitespace().collect();
    assert!(flat.contains("#[inline(always)]pubfnevent(_dev:&'staticstr,_ev:Ev){}"));
    assert!(flat.contains("#[inline(always)]pubfnsummary(_stage:&'staticstr){}"));
    assert!(MOD.contains("#[cfg(not(feature = \"nonos-bringup-trace\"))]\nmod off;"));
    assert!(CARGO.contains("\nnonos-bringup-trace = []\n"), "declared, in no default");
}

const REPORT: &str = include_str!("../../../../src/userspace/init/spawn_report.rs");
const LOOP: &str = include_str!("../../../../src/userspace/init/supervisor/loop_impl.rs");
const CENSUS_RUN: &str = include_str!("../../../../src/hardware/boot_census/run.rs");
const CENSUS_WHEN: &str = include_str!("../../../../src/hardware/boot_census/when.rs");

/// The boot census starts when init's spawns are reported and is looked at
/// on init's one-second tick; said once green or past its wait, then on
/// any change.
#[test]
fn the_census_starts_after_the_spawns_and_ticks_with_init() {
    assert!(REPORT.contains("crate::hardware::boot_census::start("));
    assert!(LOOP.contains("crate::hardware::boot_census::tick("));
    let run: String = CENSUS_RUN.split_whitespace().collect();
    assert!(run.contains(
        "pass_done:crate::hardware::broker::acpi::first_pass::done()||!facts.acpi_capsule,"
    ));
    assert!(run.contains("ifsay_now(look){"));
    let when: String = CENSUS_WHEN.split_whitespace().collect();
    assert!(when.contains("(l.green&&l.pass_done)||l.waited_ms>=WAIT_MS"));
    assert!(when.contains("pubconstWAIT_MS:u64=30_000;"));
}
