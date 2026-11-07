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

//! The reproduction and the fix. The battery is in the machine on every
//! schedule but the last few; what varies is when firmware's SMI handler
//! touches the EC and how long the EC takes to answer. A word without
//! BATTERY_PRESENT is a bar that says "No battery".

use super::super::retry::FIRST_MS;
use super::super::word::{AC_KNOWN, BATTERY_PRESENT, DISCHARGING, UNKNOWN_PERCENT};
use super::board::Board;
use super::boot::{Boot, Step};
use super::glue_tests::{BEFORE, CAPSULE};
use super::schedule::{schedule, READY_MS};
use crate::ec::TRIALS;

/// The EC's registers: discharging, AC off, battery in; 2565 of 4500 mAh.
fn charged(mut b: Board) -> Board {
    b.mem(0x40, 1);
    b.mem(0x41, 0b100);
    for (at, v) in [(0x42u8, 1500u16), (0x44, 2565), (0x46, 4500), (0x48, 5000), (0x4A, 11800)] {
        b.mem(at, v as u8);
        b.mem(at + 1, (v >> 8) as u8);
    }
    b
}

fn board(ready_ms: u64, smi_at: &[u64]) -> Board {
    charged(Board::new(ready_ms, smi_at.iter().copied().collect()))
}

/// Five minutes of the capsule's loop after its start.
const FOR_MS: u64 = 300_000;
/// The battery with its charge on every schedule within this, from the
/// capsule's start: the EC's latest start modelled (8 s) and its trials.
const BOUND_MS: u64 = 20_000;
const AT_57: u64 = 57 | BATTERY_PRESENT | DISCHARGING | AC_KNOWN;

fn present(w: u64) -> bool {
    w & BATTERY_PRESENT != 0
}

fn run(ready_ms: u64, smi_at: &[u64], order: &[Step]) -> Boot {
    Boot::run(board(ready_ms, smi_at), order, FOR_MS)
}

/// The SMIs, in order, each landing on the EC transaction right after the
/// last one's timeout: the first `n` port accesses where one costs a wait.
fn consecutive_losses(n: usize, order: &[Step]) -> Vec<u64> {
    let first = |at: &[u64]| Boot::run(board(0, at), order, 0).board.now();
    let mut at: Vec<u64> = vec![];
    for _ in 0..n {
        let base = first(&at);
        let from = at.last().copied().unwrap_or(0) + 1;
        let k = (from..from + 5_000)
            .find(|&k| first(&[&at[..], &[k]].concat()) >= base + 1_000)
            .expect("an SMI that costs a wait");
        at.push(k);
    }
    at
}

#[test]
fn a_quiet_boot_reads_the_battery() {
    let b = run(0, &[], &CAPSULE);
    assert_eq!(b.first(), AT_57, "57%, on battery");
    assert!(b.words.iter().all(|(_, w)| *w == AT_57));
}

/// The losing ordering before the fix: three SMIs between the address
/// write and the answer read of three EC reads in a row, all before ACPI
/// mode (_REG's AC read, _STA in _INI's walk, _STA in the inventory). The
/// third timeout turns the EC off; before the fix, for the boot.
#[test]
fn three_smis_in_the_old_order_now_cost_seconds_not_the_boot() {
    let at = consecutive_losses(3, &BEFORE);
    let b = run(0, &at, &BEFORE);
    eprintln!("SMIs before port accesses {at:?}; steps began at {:?}", b.began);
    assert!(b.board.smis == 3 && b.board.health.trials() == 1, "off, then on trial");
    assert!(b.words.iter().all(|(_, w)| present(*w)), "never 'no battery': {:?}", b.words);
    assert!(b.charged_at().is_some_and(|t| t <= BOUND_MS), "{:?}", b.charged_at());
}

/// ACPI mode entered before _REG: the same SMIs find no EC access to land
/// in, and the first report has the charge.
#[test]
fn acpi_mode_first_leaves_the_smis_nothing_to_take() {
    let at = consecutive_losses(3, &BEFORE);
    let b = run(0, &at, &CAPSULE);
    assert!(b.board.health.usable() && b.board.smis == 0 && b.board.health.trials() == 0);
    assert_eq!(b.first(), AT_57);
}

/// An EC that starts late: the battery is reported present with its charge
/// unknown (the bar says "Battery status unavailable", not "No battery")
/// until the EC answers, then with its charge.
#[test]
fn a_slow_ec_start_costs_seconds_not_the_boot() {
    for ready in READY_MS {
        let b = run(ready, &[], &CAPSULE);
        assert!(b.words.iter().all(|(_, w)| present(*w)), "ready {ready}: {:?}", b.words);
        let at = b.charged_at().expect("charged");
        assert!(at <= BOUND_MS && at >= ready.min(at), "ready {ready}: charged at {at}");
        assert_eq!(b.words.last().map(|w| w.1), Some(AT_57));
    }
    let b = run(3_500, &[], &CAPSULE);
    assert_eq!(b.first(), UNKNOWN_PERCENT | BATTERY_PRESENT, "present, charge unknown");
}

/// Every seeded schedule, under the capsule's order and the old one: never
/// "no battery", and the charge within the bound.
#[test]
fn every_schedule_has_the_battery_within_its_bound() {
    for order in [CAPSULE, BEFORE] {
        let window = Boot::run(board(0, &[]), &order, 0).board.ops;
        let (mut first, mut worst) = (0, 0);
        for seed in 0..600 {
            let s = schedule(seed, window);
            let b = Boot::run(charged(s.board()), &order, FOR_MS);
            assert!(b.words.iter().all(|(_, w)| present(*w)), "seed {seed}: {:?}", b.words);
            let at = b.charged_at().unwrap_or_else(|| panic!("seed {seed} never charged"));
            assert!(at <= BOUND_MS, "seed {seed}: charged at {at} ms");
            first += usize::from(b.first() == AT_57);
            worst = worst.max(at);
        }
        eprintln!("{order:?}: charge on the first report {first}/600, latest {worst} ms");
    }
}

/// A machine with no battery device (a desktop, a VM) and a laptop with
/// its slot empty: absent on the first report, with no reading owed.
#[test]
fn no_battery_is_absent_at_once_and_owes_nothing() {
    let desk = Boot::run_on(Boot::with(board(0, &[]), false), &CAPSULE, FOR_MS);
    assert!(desk.words.iter().all(|(_, w)| !present(*w)));
    assert!(!desk.power.retry.due(u64::MAX) && desk.power.batteries.is_empty());
    let mut empty = board(0, &[]);
    empty.mem(0x41, 0);
    let slot = Boot::run(empty, &CAPSULE, FOR_MS);
    assert!(slot.words.iter().all(|(_, w)| !present(*w)), "{:?}", slot.words);
    assert!(!slot.power.retry.due(u64::MAX), "an answered _STA owes nothing");
    assert_eq!(slot.words.len(), 1 + 5, "the first read and the minute's polls only");
}

/// An EC that never answers: never "no battery", the trials and re-reads
/// end within their bounds, and the loop goes on.
#[test]
fn a_dead_ec_ends_its_trials_and_never_says_absent() {
    let b = run(u64::MAX, &[], &CAPSULE);
    assert!(!b.board.health.usable() && b.board.health.trials() == TRIALS);
    assert!(b.words.iter().all(|(_, w)| *w == UNKNOWN_PERCENT | BATTERY_PRESENT));
    assert!(!b.power.retry.due(u64::MAX), "the re-reads ended");
    let owed: u64 = (0..8).map(|i| FIRST_MS << i).sum();
    assert!(owed < 70_000, "about a minute of re-reads, then the minute's poll");
}

/// A battery put in after boot: its Notify, or the minute's poll at the
/// latest, finds it.
#[test]
fn a_battery_put_in_later_is_found_by_the_poll() {
    let mut empty = board(0, &[]);
    empty.mem(0x41, 0);
    let mut b = Boot::run(empty, &CAPSULE, 30_000);
    assert!(!present(b.first()));
    b.board.mem(0x41, 0b100);
    let b = Boot::run_on(b, &[], 61_000);
    assert_eq!(b.words.last().map(|w| w.1), Some(AT_57));
}

const FIRST: &str = include_str!("../../../../../capsule_acpi/src/boot/first.rs");
const WAIT: &str = include_str!("../../../../../../src/hardware/broker/acpi/first_pass_rule.rs");

/// An EC that starts late holds the capsule before its first publish pass:
/// _REG's, _INI's and the inventory's EC reads each wait their second out.
/// That is 3 s; with the pass's own 2 s budget for hidden functions it
/// reaches init's 5 s wait, and any time the capsule spent loading its
/// tables first puts it past. So what the pass publishes must not depend on
/// that wait (init's late walk, src/userspace/init/spawn_plan/late.rs).
#[test]
fn a_late_ec_can_put_the_first_pass_past_inits_wait() {
    assert!(FIRST.contains("pub const FIRST_PASS_MS: u64 = 2_000;"));
    assert!(WAIT.contains("pub const FIRST_PASS_WAIT_MS: u64 = 5_000;"));
    for ready in [3_500, 8_000] {
        let b = Boot::run(board(ready, &[]), &CAPSULE, 0);
        let pass = b.began_ms.iter().find(|(s, _)| *s == Step::ReadPower).map(|(_, t)| *t);
        let pass = pass.expect("the inventory ended");
        eprintln!("EC ready at {ready} ms: the first pass starts at {pass} ms");
        assert!(pass >= 3_000 && pass + 2_000 >= 5_000, "ready {ready}: {pass} ms");
    }
    let quiet = Boot::run(board(0, &[]), &CAPSULE, 0);
    assert!(quiet.began_ms.iter().all(|(_, t)| *t < 100), "an EC that answers costs nothing");
}
