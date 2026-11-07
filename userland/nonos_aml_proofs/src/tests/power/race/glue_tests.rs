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

//! The orders `boot` replays are the capsule's own, and the board's EC
//! access is capsule_acpi's host/ec.rs: each pinned to the source text.

use super::boot::Step::{self, *};

const START: &str = include_str!("../../../../../capsule_acpi/src/boot/start.rs");
const LATE: &str = include_str!("../../../../../capsule_acpi/src/boot/late.rs");
const SCI: &str = include_str!("../../../../../capsule_acpi/src/boot/sci.rs");
const HOST_EC: &str = include_str!("../../../../../capsule_acpi/src/host/ec.rs");
const RUN: &str = include_str!("../../../../../capsule_acpi/src/server/run.rs");
const EC_RETRY: &str = include_str!("../../../../../capsule_acpi/src/ec/retry.rs");
const REFRESH: &str = include_str!("../../../../../capsule_acpi/src/power/refresh.rs");

/// The order capsule_acpi brings the platform up in: ACPI mode first.
pub const CAPSULE: [Step; 5] = [AcpiMode, Connect, Ini, Inventory, ReadPower];
/// The order before this fix, ACPI mode in `late`.
pub const BEFORE: [Step; 5] = [Connect, Ini, Inventory, AcpiMode, ReadPower];

fn at(src: &str, what: &str) -> usize {
    src.find(what).unwrap_or_else(|| panic!("{what} is in the source"))
}

fn flat(s: &str) -> String {
    s.split_whitespace().collect()
}

#[test]
fn the_replayed_order_is_the_capsules() {
    // ACPI mode, then _REG and _INI (bring_up), the inventory, `late`.
    assert!(at(START, "sci::enter(") < at(START, "bringup::bring_up("));
    assert!(at(START, "bringup::bring_up(") < at(START, "report::read("));
    assert!(at(START, "report::read(") < at(START, "late::late("));
    assert!(at(LATE, "sci::start(") < at(LATE, "crate::power::all("));
    assert!(at(SCI, "super::acpi_mode::enter_mode(&f);") < at(SCI, "pub fn start("));
}

#[test]
fn the_boards_ec_access_is_the_capsules() {
    let flat = flat(HOST_EC);
    let off = "letSome((ports,glk))=h.ec.filter(|_|h.ec_health.usable())else{";
    assert!(flat.contains(off), "refused while off");
    assert!(flat.contains("ec::read(&ports,&mutio,addr)"), "the handshake");
    assert!(flat.contains("ifh.ec_health.note(r){"), "each answer counted");
}

/// The loop `Boot::run` replays: the EC tried again before the power
/// tick, a trial reading the batteries at once, and a reading owed or a
/// minute passed reading them again.
#[test]
fn the_replayed_loop_is_the_capsules() {
    assert!(at(RUN, "crate::ec::retry(&mut acpi);") < at(RUN, "crate::power::tick("));
    assert!(flat(EC_RETRY).contains("!acpi.host.ec_health.retry(now_ms())"));
    assert!(EC_RETRY.contains("acpi.power.read_ms = 0;"));
    let r = flat(REFRESH);
    assert!(r.contains("ifnow.saturating_sub(p.read_ms)>=every||p.retry.due(now){"));
    assert!(r.contains("p.retry.after(super::retry::wanted(p),p.read_ms);"));
}
