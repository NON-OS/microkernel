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

//! The one tool outside the subset bound: `tool.model-fetch`, matched whole.
//! Every other name, each tool the registry bakes in among them, is bounded;
//! and the fetcher's own caller gate (Network and FileSystem) stays.

use crate::stand_in::world;
use crate::tool_bound::{check, EXEMPT};

const RUN: &str = include_str!("../../../../src/userspace/tool_capsules/run.rs");
const REGISTRY: &str = include_str!("../../../../src/userspace/tool_capsules/registry.rs");
const FETCH: &str = include_str!("../../../../src/userspace/tool_capsules/model_fetch/spawn.rs");
const IPC_ONLY: u64 = 0x1 | 0x8;
const WIDE: u64 = 0x0000_0004_0000_003D;

fn bounded(name: &[u8]) -> bool {
    world(Some(9), &[(9, IPC_ONLY)]);
    let manifest = [WIDE.to_be_bytes(), 0u64.to_be_bytes()].concat();
    check(&manifest, WIDE, name).is_err()
}

#[test]
fn exactly_one_name_is_exempt_and_only_whole() {
    assert_eq!(EXEMPT, [&b"tool.model-fetch"[..]]);
    assert!(!bounded(b"tool.model-fetch"));
    for near in ["tool.model-fetchX", "tool.model-", "tool.model-fetc", "Tool.model-fetch", ""] {
        assert!(bounded(near.as_bytes()), "{near:?} is bounded");
    }
}

#[test]
fn every_other_tool_name_is_bounded() {
    let mut names = vec!["tool.linux", "tool.qwen", "tool.install"];
    for chunk in REGISTRY.split("tool_capsule!(").skip(1) {
        names.push(chunk.split('"').nth(1).expect("a tool's name"));
    }
    assert!(names.len() > 8, "the registry's tools were found: {names:?}");
    for name in names {
        assert!(bounded(name.as_bytes()), "{name} goes through the subset bound");
    }
}

#[test]
fn the_fetcher_keeps_its_caller_gate_and_is_checked_by_name() {
    let code: String = FETCH.split_whitespace().collect();
    let gate = code.find("if!(caller.can_network()&&caller.can_open_files()){returnErr(ERRNO_PERM);}");
    let spawn = code.find("capsule_spawn::spawn_verified(");
    assert!(gate.expect("the Network and FileSystem gate") < spawn.expect("the spawn"));
    let run: String = RUN.split_whitespace().collect();
    let check = run.find("super::bound::check(super::model_fetch::MANIFEST,super::model_fetch::CAPS,name)?;");
    assert!(check.expect("checked by name") < run.find("model_fetch::run_for_caller(argv)").unwrap());
}
