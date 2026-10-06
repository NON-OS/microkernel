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

//! A dock launch starts a numbered instance ("app.terminal.2"), and every
//! place the shell asks which app a window is must count it as its app: the
//! running mark, the window a dock click raises, the menubar title and the
//! open and close toasts.

use crate::instance::{
    app_index_of, instance_name, is_instance_of, INSTANCE_NAME_MAX, INSTANCE_SLOTS,
};
use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::{
    new_taskbar_state, raise_tracked, track_window_closed, track_window_opened,
    Reach, TASKBAR_NO_ACTIVE,
};

fn bases() -> impl Iterator<Item = &'static [u8]> {
    LAUNCHER_APPS.iter().map(|a| a.service)
}

fn index_of(service: &[u8]) -> usize {
    LAUNCHER_APPS.iter().position(|a| a.service == service).unwrap()
}

#[test]
fn the_base_and_its_numbered_instances_match() {
    assert!(is_instance_of(b"app.terminal", b"app.terminal"));
    assert!(is_instance_of(b"app.terminal", b"app.terminal.1"));
    assert!(is_instance_of(b"app.terminal", b"app.terminal.2"));
    assert!(is_instance_of(b"app.terminal", b"app.terminal.10"));
}

#[test]
fn a_longer_name_or_a_non_numeric_tail_does_not() {
    for name in [
        &b"app.terminal_x"[..],
        b"app.terminalfoo",
        b"app.terminal.",
        b"app.terminal.2b",
        b"app.terminal.x",
        b"app.terminal..2",
        b"app.terminal.-1",
        b"app.termina",
        b"app.terminal_x.2",
        b"app.text_editor",
    ] {
        assert!(!is_instance_of(b"app.terminal", name), "{:?}", core::str::from_utf8(name));
    }
}

#[test]
fn instance_names_are_built_as_the_kernel_declares_them() {
    let mut buf = [0u8; INSTANCE_NAME_MAX];
    assert_eq!(instance_name(b"app.terminal", 0, &mut buf), Some(&b"app.terminal"[..]));
    assert_eq!(instance_name(b"app.terminal", 2, &mut buf), Some(&b"app.terminal.2"[..]));
    assert_eq!(instance_name(b"app.browser", 12, &mut buf), Some(&b"app.browser.12"[..]));
    let long = [b'a'; INSTANCE_NAME_MAX - 1];
    assert_eq!(instance_name(&long, 1, &mut buf), None);
}

#[test]
fn every_probed_name_maps_back_to_its_own_app() {
    let mut buf = [0u8; INSTANCE_NAME_MAX];
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        for slot in 0..=INSTANCE_SLOTS {
            let name = instance_name(app.service, slot, &mut buf).unwrap();
            assert_eq!(app_index_of(bases(), name), Some(index));
        }
    }
}

/// The names the kernel's signed spawn tables register extra windows under,
/// read from the kernel source, so a table that changes its naming fails here.
#[test]
fn the_kernels_instance_names_are_matched_to_their_app() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../src/userspace");
    let mut seen = 0;
    for entry in std::fs::read_dir(root).unwrap() {
        let spawn = entry.unwrap().path().join("spawn.rs");
        let Ok(text) = std::fs::read_to_string(&spawn) else { continue };
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("name: \"app.") else { continue };
            let name = format!("app.{}", rest.trim_end_matches("\",").trim_end_matches('"'));
            let Some((base, slot)) = name.rsplit_once('.') else { continue };
            if !slot.bytes().all(|b| b.is_ascii_digit()) {
                continue;
            }
            assert!(slot.parse::<u32>().unwrap() <= INSTANCE_SLOTS, "{name}");
            if let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == base.as_bytes()) {
                assert_eq!(app_index_of(bases(), name.as_bytes()), Some(index), "{name}");
                seen += 1;
            }
        }
    }
    assert!(seen > 10, "found only {seen} instance names");
}

#[test]
fn a_second_window_closing_leaves_the_app_running() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 11, 101, term);
    assert_eq!(track_window_closed(&mut t, 101, 11), Some(term));
    assert!(t.open[term]);
    assert_eq!(t.active as usize, term);
    assert_eq!(track_window_closed(&mut t, 100, 10), Some(term));
    assert!(!t.open[term]);
    assert_eq!(t.active, TASKBAR_NO_ACTIVE);
}

#[test]
fn closing_the_first_window_keeps_the_later_instance() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 11, 101, term);
    track_window_closed(&mut t, 100, 10);
    assert!(t.open[term]);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
}

#[test]
fn a_dock_click_raises_the_newest_window_of_that_app() {
    let term = index_of(b"app.terminal");
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 20, 200, files);
    track_window_opened(&mut t, 11, 101, term);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
    assert_eq!(raise_tracked(&mut t, files, |_| Reach::Taken), Some(200));
    track_window_closed(&mut t, 200, 20);
    assert_eq!(raise_tracked(&mut t, files, |_| Reach::Taken), None);
}

#[test]
fn the_menubar_follows_the_app_whose_window_opened_last() {
    let term = index_of(b"app.terminal");
    let files = index_of(b"app.file_manager");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 20, 200, files);
    assert_eq!(t.active as usize, files);
    track_window_closed(&mut t, 200, 20);
    assert_eq!(t.active as usize, term);
}

#[test]
fn a_close_the_shell_never_saw_open_changes_nothing() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    assert_eq!(track_window_closed(&mut t, 999, 99), None);
    assert!(t.open[term]);
}

#[test]
fn an_opened_event_repeated_for_one_window_counts_once() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 10, 100, term);
    track_window_closed(&mut t, 100, 10);
    assert!(!t.open[term]);
}

/* Every instance of an app opens a window with the id in its manifest, so
 * two terminals hold window 10 each. A second open must not evict the first,
 * and a close must take only its own. */
#[test]
fn instances_sharing_a_window_id_are_told_apart_by_pid() {
    let term = index_of(b"app.terminal");
    let mut t = new_taskbar_state();
    track_window_opened(&mut t, 10, 100, term);
    track_window_opened(&mut t, 10, 101, term);
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(101));
    assert_eq!(track_window_closed(&mut t, 101, 10), Some(term));
    assert!(t.open[term], "the first terminal is still up");
    assert_eq!(raise_tracked(&mut t, term, |_| Reach::Taken), Some(100));
    assert_eq!(track_window_closed(&mut t, 101, 10), None, "already gone");
    assert_eq!(track_window_closed(&mut t, 100, 10), Some(term));
    assert!(!t.open[term]);
}

/* The image viewer is in the table, so a picture opens in it and its windows
 * count as its app, but it has no dock tile: the dock's apps lead the table
 * and the viewer comes after them. */
#[test]
fn the_image_viewer_is_launchable_but_off_the_dock() {
    use crate::shell_apps::DOCK_APPS;
    let viewer = index_of(b"app.image_viewer");
    assert!(!LAUNCHER_APPS[viewer].dock);
    assert!(viewer >= DOCK_APPS);
    assert!(LAUNCHER_APPS[..DOCK_APPS].iter().all(|a| a.dock));
    assert!(LAUNCHER_APPS[DOCK_APPS..].iter().all(|a| !a.dock));
    assert_eq!(DOCK_APPS, LAUNCHER_APPS.len() - 1);
}
