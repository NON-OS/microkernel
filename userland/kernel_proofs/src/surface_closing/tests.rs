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

//! The exit window, as keep_at_exit, attach_surface and
//! release_owned_by_pid run it under GATE.

use super::closing_rule::{admits, close, forget, Closing};

const APP: u32 = 21;
const OTHER: u32 = 22;
const WINDOW: u32 = 0x31;
const BACKING: u32 = 0x32;
const OTHERS: u32 = 0x40;

#[test]
fn an_attach_inside_the_exit_window_is_refused() {
    let mut list: Closing<u32> = Vec::new();
    assert!(admits(&list, WINDOW), "before the exit, attachable");
    close(&mut list, APP, &[WINDOW, BACKING]);
    assert!(!admits(&list, WINDOW) && !admits(&list, BACKING), "inside the window");
    assert!(admits(&list, OTHERS), "another process's surface is untouched");
    forget(&mut list, APP);
    assert!(list.is_empty(), "the slots are gone; nothing left to hold");
}

#[test]
fn two_exits_close_and_forget_only_their_own() {
    let mut list: Closing<u32> = Vec::new();
    close(&mut list, APP, &[WINDOW]);
    close(&mut list, OTHER, &[OTHERS]);
    forget(&mut list, APP);
    assert!(admits(&list, WINDOW), "APP's handle is stale now, the epoch moved on");
    assert!(!admits(&list, OTHERS), "OTHER is still exiting");
}

#[test]
fn an_exit_with_no_surfaces_closes_nothing() {
    let mut list: Closing<u32> = Vec::new();
    close(&mut list, APP, &[]);
    assert!(list.is_empty() && admits(&list, WINDOW));
}
