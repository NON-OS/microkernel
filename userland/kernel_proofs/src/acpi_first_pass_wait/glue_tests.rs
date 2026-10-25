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
//! Where init asks the rule, in files the host cannot run: the wait sits
//! between the ACPI capsule's spawn and every driver family after it, it
//! asks wait_needed before it waits, and its bound covers the capsule's.

use crate::hardware::broker::acpi::first_pass_rule::FIRST_PASS_WAIT_MS;

const ORCHESTRATOR: &str =
    include_str!("../../../../src/userspace/init/spawn_plan/orchestrator.rs");
const PLATFORM: &str = include_str!("../../../../src/userspace/init/spawn_plan/platform_acpi.rs");
const WAIT: &str = include_str!("../../../../src/arch/x86_64/acpi/first_pass_wait.rs");
const CAPSULE_FIRST: &str = include_str!("../../../../userland/capsule_acpi/src/boot/first.rs");

#[test]
fn only_virtio_spawns_before_the_acpi_capsule_and_its_wait() {
    let body = ORCHESTRATOR.split("fn spawn_drivers() {").nth(1).expect("spawn_drivers");
    let body = &body[..body.find("\n}").expect("end of spawn_drivers")];
    let calls: Vec<&str> =
        body.lines().map(str::trim).filter(|l| l.starts_with("super::")).collect();
    assert_eq!(&calls[..2], ["super::drivers_virtio::spawn();", "super::platform_acpi::spawn();"]);
    assert!(calls.len() > 2, "the drivers come after the wait");
    assert!(PLATFORM.contains("crate::arch::acpi_first_pass_wait(spawn_acpi());"));
    assert!(PLATFORM.contains("c::shared_state().is_alive()"), "spawned means it is alive");
}

#[test]
fn the_wait_asks_the_rule_first_and_looks_each_round() {
    let needed = WAIT
        .find("if !rule::wait_needed(super::is_initialized(), spawned, done()) {")
        .expect("gate");
    let looks = WAIT.find("match rule::look(done(), waited) {").expect("look");
    assert!(needed < looks, "nothing waits on a boot that does not need it");
    assert!(WAIT.contains("rule::Look::Wait => crate::sched::yield_now(),"));
    assert_eq!(WAIT.matches("return boot_log::").count(), 2, "said once, either way");
}

#[test]
fn the_bound_covers_the_capsules_own_reveal_budget_twice() {
    let line =
        CAPSULE_FIRST.lines().find(|l| l.contains("pub const FIRST_PASS_MS: u64 =")).unwrap();
    let digits: String =
        line.split('=').nth(1).unwrap().chars().filter(char::is_ascii_digit).collect();
    let budget: u64 = digits.parse().unwrap();
    assert!(
        FIRST_PASS_WAIT_MS >= 2 * budget,
        "{FIRST_PASS_WAIT_MS} ms against the capsule's {budget}"
    );
}
