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

//! A window run names its package to the desktop shell before its guest
//! exists (window_tile.rs), by an op and a length the shell must read the
//! same way: hand-synced across two crates, so held equal here.

const TILE: &str = include_str!("../../../capsule_linux/src/linux/window_tile.rs");
const START: &str = include_str!("../../../capsule_linux/src/linux/start.rs");
const SHELL_OPS: &str = include_str!("../../../capsule_desktop_shell/src/protocol/ops.rs");
const SHELL_RUN: &str =
    include_str!("../../../capsule_desktop_shell/src/state/taskbar/linux_run.rs");
const SHELL_HANDLER: &str =
    include_str!("../../../capsule_desktop_shell/src/server/handlers/linux_window.rs");

#[test]
fn the_op_is_the_shells() {
    assert!(TILE.contains("const OP_LINUX_WINDOW: u16 = 0x0013;"));
    assert!(SHELL_OPS.contains("pub const OP_LINUX_WINDOW: u16 = 0x0013;"));
    assert!(TILE.contains("const NDSH: u32 = 0x4E44_5348;"));
}

#[test]
fn the_longest_name_is_the_shells() {
    assert!(TILE.contains("const PACKAGE_MAX: usize = 64;"));
    assert!(SHELL_RUN.contains("pub const LINUX_PACKAGE_MAX: usize = 64;"));
}

/// Named before the guest is spawned, so the window's open notice finds the
/// tile; and only by a run in its own window.
#[test]
fn named_before_the_guest_and_only_for_a_window() {
    let named = START.find("window_tile::name_to_shell()").expect("named at start");
    let spawned = START.find("mk_foreign_spawn(b\"linux\")").expect("guest spawned");
    assert!(named < spawned);
    assert!(TILE.contains("let Some((name, Mode::Window)) = super::request::run_request()"));
}

/// The shell believes the name only from the run role, before it reads it.
#[test]
fn the_shell_takes_it_only_from_the_run_role() {
    let perm = SHELL_HANDLER.find("lookup_pid(RUN) != Some(sender_pid)").expect("role check");
    let noted = SHELL_HANDLER.find("run_named(").expect("noted");
    assert!(perm < noted);
}
