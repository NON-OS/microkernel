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

//! A Linux run names its package as it starts (state/taskbar/linux_run.rs):
//! a Qwen tier awaits its window as Qwen's whoever asked for the run, so a
//! Qwen opened from the Terminal's `qwen window` comes back from its dock
//! tile too, and any other package ends the wait, so a store run that took
//! the role ahead of the shell's queued Qwen is not taken for Qwen.

use super::rig::index_of;
use crate::taskbar::{
    expect_run_window, new_taskbar_state, raise_tracked, run_named, run_window_app,
    track_window_opened, Reach, LINUX_PACKAGE_MAX,
};

const RUN_PID: u32 = 72;
const QWEN_WINDOW: u32 = 0x10;

fn qwen() -> usize {
    index_of(b"tool.qwen")
}

#[test]
fn every_tier_package_names_the_qwen_tile() {
    for package in [&b"qwen-small"[..], b"qwen-qwen3-0.6b", b"qwen-qwen3-30b-a3b", b"qwen-vl-2b"] {
        let mut t = new_taskbar_state();
        assert!(run_named(&mut t, package), "{:?}", package);
        assert_eq!(run_window_app(&mut t, RUN_PID, || Some(RUN_PID)), Some(qwen()));
    }
}

/// Every tier the personality runs a window for (`qwen-<word>`, its
/// terminal/tier.rs) names the Qwen tile.
#[test]
fn the_personalitys_own_tiers_name_the_qwen_tile() {
    const TIERS: &str = include_str!("../../../../../src/userspace/capsule_linux/terminal/tier.rs");
    let list = TIERS.split("const TIERS").nth(1).expect("the tier list");
    let list = &list[..list.find("];").expect("its end")];
    let words: Vec<&str> = list.split('"').skip(1).step_by(2).collect();
    assert_eq!(words.len(), 20);
    for word in words {
        let package = format!("qwen-{word}");
        let mut t = new_taskbar_state();
        assert!(run_named(&mut t, package.as_bytes()), "{package}");
    }
}

#[test]
fn a_name_that_is_no_tier_names_nothing() {
    let long = [&b"qwen-"[..], &[b'a'; LINUX_PACKAGE_MAX][..]].concat();
    for package in [
        &b""[..],
        b"qwen",
        b"qwen-",
        b"telegram-desktop",
        b"kali.qwen-small",
        b"Qwen-small",
        b"qwen-Small",
        b"qwen-small/../x",
        b"qwen-small\0",
        b"qwen-sm all",
        &long[..],
    ] {
        let mut t = new_taskbar_state();
        assert!(!run_named(&mut t, package), "{:?}", package);
        assert_eq!(run_window_app(&mut t, RUN_PID, || Some(RUN_PID)), None, "{:?}", package);
    }
}

/// The shell queued Qwen, but a store Open's run took the role first: that
/// run names its own package, so its window is no app's and not Qwen's.
#[test]
fn a_store_run_ahead_of_a_queued_qwen_is_not_qwen() {
    let mut t = new_taskbar_state();
    expect_run_window(&mut t, qwen());
    assert!(!run_named(&mut t, b"telegram-desktop"));
    assert_eq!(run_window_app(&mut t, RUN_PID, || Some(RUN_PID)), None);
}

/// Qwen opened from the Terminal: the shell queued nothing, the run names
/// its tier, and its window is tracked as Qwen's, so the tile raises it
/// once minimised instead of asking for a second run.
#[test]
fn a_qwen_the_shell_did_not_queue_comes_back_from_the_dock() {
    let mut t = new_taskbar_state();
    assert!(run_named(&mut t, b"qwen-qwen3-0.6b"));
    let app = run_window_app(&mut t, RUN_PID, || Some(RUN_PID)).expect("Qwen's tile");
    track_window_opened(&mut t, QWEN_WINDOW, RUN_PID, app);
    assert!(t.open[qwen()]);
    let raised = raise_tracked(&mut t, qwen(), |_| Reach::Taken);
    assert_eq!(raised, Some(RUN_PID), "the focus frame goes to the run");
}
