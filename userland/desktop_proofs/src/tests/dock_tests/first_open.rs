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
//! One Terminal process for one window: an app with no window open is
//! opened by its boot process when that takes the open frame, and only
//! otherwise by a new instance.

use crate::taskbar::{first_open, FirstOpen};
use core::cell::Cell;

const BOOT_TERMINAL: u32 = 0x3B;

#[test]
fn the_boot_process_takes_the_first_window() {
    let sent = Cell::new(None);
    let take = |pid| {
        sent.set(Some(pid));
        true
    };
    assert_eq!(first_open(Some(BOOT_TERMINAL), take), FirstOpen::Woken);
    assert_eq!(sent.get(), Some(BOOT_TERMINAL), "the frame went to the boot process");
}

#[test]
fn a_new_instance_only_when_the_boot_process_cannot_open_it() {
    assert_eq!(first_open(Some(BOOT_TERMINAL), |_| false), FirstOpen::Spawn, "gone or busy");
    let asked = Cell::new(false);
    let take = |_| {
        asked.set(true);
        true
    };
    assert_eq!(first_open(None, take), FirstOpen::Spawn, "no boot process");
    assert!(!asked.get(), "nothing is sent without a pid");
}
