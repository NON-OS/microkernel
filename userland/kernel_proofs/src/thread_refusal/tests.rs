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

use super::thread_start::{start_in_user_half, USER_VA_MAX};

const BUILDER: &str = include_str!("../../../../src/arch/x86_64/context/setup.rs");
const SPAWN: &str = include_str!("../../../../src/process/core/table/thread_spawn.rs");
const MK_THREAD: &str = include_str!("../../../../src/syscall/microkernel/process.rs");
const FOREIGN_THREAD: &str = include_str!("../../../../src/process/foreign/thread.rs");

/// The bound the x86_64 user entry builder states, read from its source.
fn builder_bound() -> u64 {
    let line = BUILDER
        .lines()
        .find_map(|l| l.trim().strip_prefix("const USER_VA_MAX: u64 = "))
        .expect("the builder states its bound");
    let digits = line.trim_end_matches(';').trim_start_matches("0x").replace('_', "");
    u64::from_str_radix(&digits, 16).expect("a hex bound")
}

/// What the builder accepts, written from its two refusals.
fn builder_accepts(entry: u64, rsp: u64) -> bool {
    let max = builder_bound();
    let refuses = |v: u64| v == 0 || v > max;
    !refuses(entry) && !refuses(rsp)
}

/// Byte offset of `needle` in `text`, which must hold it.
fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn refuses_a_start_outside_the_user_half() {
    let kernel = 0xFFFF_8000_0000_0000;
    for (entry, stack) in [
        (0, 0x7000_0000),
        (0x40_0000, 0),
        (USER_VA_MAX + 1, 0x7000_0000),
        (0x40_0000, USER_VA_MAX + 1),
        (kernel, 0x7000_0000),
        (0x40_0000, kernel),
        (u64::MAX, u64::MAX),
    ] {
        assert!(!start_in_user_half(entry, stack), "{entry:#x} on {stack:#x}");
    }
}

#[test]
fn admits_every_start_inside_it() {
    for (entry, stack) in [(1, 1), (0x40_0000, 0x7FFF_FFFF_F000), (USER_VA_MAX, USER_VA_MAX)] {
        assert!(start_in_user_half(entry, stack), "{entry:#x} on {stack:#x}");
    }
}

#[test]
fn the_bound_is_the_builders_and_the_user_copies() {
    assert_eq!(USER_VA_MAX, builder_bound());
    assert_eq!(USER_VA_MAX, crate::usercopy::USER_SPACE_END);
    assert!(BUILDER.contains("if entry == 0 || entry > USER_VA_MAX {"));
    assert!(BUILDER.contains("if user_rsp == 0 || user_rsp > USER_VA_MAX {"));
}

#[test]
fn agrees_with_the_builder_at_every_edge() {
    let max = builder_bound();
    let edges = [0, 1, 0x1000, max - 1, max, max + 1, 1 << 48, 0xFFFF_8000_0000_0000, u64::MAX];
    for entry in edges {
        for stack in edges {
            assert_eq!(
                start_in_user_half(entry, stack),
                builder_accepts(entry, stack),
                "{entry:#x} on {stack:#x}"
            );
        }
    }
}

#[test]
fn the_spawn_asks_before_it_builds_and_ends_what_it_cannot_start() {
    let body = &SPAWN[at(SPAWN, "pub fn spawn_thread_parked")..];
    assert!(at(body, "start_in_user_half(entry, stack)") < at(body, "allocate_tid()"));
    let published = &body[at(body, "PROCESS_TABLE.add(pcb);")..];
    let tail = &published[..at(published, "Ok(tid)")];
    assert!(!tail.contains(")?;"), "a step after publishing returns past the thread");
    assert!(tail.contains("return Err(unstarted(tid, \"thread kernel stack\"));"));
    assert!(tail.contains("return Err(unstarted(tid, \"thread user context\"));"));
    let undo = &SPAWN[at(SPAWN, "fn unstarted(")..];
    assert!(undo[..at(undo, "\n}")].contains("crate::process::exit::teardown(tid, "));
}

#[test]
fn both_thread_calls_ask_first() {
    let mk = &MK_THREAD[at(MK_THREAD, "pub fn sys_thread_spawn")..];
    assert!(at(mk, "start_in_user_half(entry, stack)") < at(mk, "spawn_thread(entry, stack)"));
    let foreign = &FOREIGN_THREAD[at(FOREIGN_THREAD, "pub fn sys_foreign_thread")..];
    assert!(at(foreign, "start_in_user_half(entry, rsp)") < at(foreign, "spawn_thread_parked("));
}

/// A thread is its process: it carries the process's capabilities, not the
/// ambient bound a child gets, and it has an inbox its IPC answers reach.
#[test]
fn a_thread_keeps_its_process_caps_and_gets_a_reply_inbox() {
    let code: String = SPAWN.lines().filter(|l| !l.trim_start().starts_with("//") && !l.trim_start().starts_with('*')).collect::<Vec<_>>().join("\n");
    assert!(code.contains("let caps = parent.caps_bits.load(Ordering::Acquire);"));
    assert!(!code.contains("compute_inherited_caps"));
    let add = code.find("PROCESS_TABLE.add(pcb);").expect("the thread is published");
    let inbox = code.find("register_inbox(&alloc::format!(\"proc.{}\", tid), tid)").expect("its inbox");
    assert!(inbox > add, "the inbox is made once the thread is in the table");
}
