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

//! The power-only build: every part that acts is off and gated where it
//! runs, every op that acts answers EPERM, the rest answer, and every
//! image builds it.

use super::index::index;
use super::protocol::*;
use super::rule::{op_runs, runs, ACTING_OPS};
use super::sites::{gate, REPLY, SITES};

const MK: &str = include_str!("../../../capsule_acpi/Capsule.mk");

fn flat(s: &str) -> String {
    s.split_whitespace().collect()
}

#[test]
fn every_part_is_gated_on_in_full_and_off_in_power_only() {
    let mut gated: Vec<usize> = SITES.iter().map(|s| index(s.0)).collect();
    gated.dedup();
    assert_eq!(gated, (0..17).collect::<Vec<_>>(), "every part has a gate, in order");
    for (p, ..) in SITES {
        assert!(runs(false, p) && !runs(true, p), "{p:?}: full runs it, power-only not");
    }
}

#[test]
fn each_gate_guards_its_call() {
    for (p, file, g, call) in SITES {
        let (src, g) = (flat(file), gate(file, g));
        let at = src.find(&g).unwrap_or_else(|| panic!("{p:?}: no gate {g}"));
        let block = &src[at + g.len()..];
        let end = block.find('}').unwrap_or(block.len());
        assert!(block[..end].contains(call), "{p:?}: {call} not under its gate");
    }
}

#[test]
fn power_only_refuses_exactly_the_ops_that_act() {
    let acting = [OP_SET_BRIGHTNESS, OP_PREPARE_SLEEP, OP_DEVICE_POWER, OP_GPIO_EVENT];
    assert_eq!(ACTING_OPS, [acting[0], acting[1], acting[2], acting[3], OP_SET_POWER_CAP]);
    for op in 0..=255u8 {
        assert!(op_runs(false, op), "op {op} in a full build");
        assert_eq!(op_runs(true, op), !ACTING_OPS.contains(&op), "op {op}");
    }
    for op in [OP_STATUS, OP_POWER, OP_THERMAL, OP_SMBIOS, OP_BRIGHTNESS, OP_LEGACY] {
        assert!(op_runs(true, op), "op {op} reads, and answers");
    }
    let gate = "if!admits(op,sender,owner,group_of)||!crate::release::op_on(op){returnEPERM";
    assert!(flat(REPLY).contains(gate), "the op gate answers EPERM before any op runs");
}

#[test]
fn every_image_builds_the_power_only_capsule() {
    let line = MK.lines().find(|l| l.starts_with("CAPSULE_CARGO_FEATURES"));
    let features = line.and_then(|l| l.split(":=").nth(1)).map(str::trim);
    assert_eq!(features, Some("release-power-only"));
}
