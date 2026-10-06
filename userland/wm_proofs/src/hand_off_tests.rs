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

//! Focus after the focused window goes: closed, minimised or its process
//! dead. The window manager's close, minimise and dead sweep take the window
//! out of the table or off the screen and then call `hand_off`, as here.
//! Before, each of them cleared focus, and the input router sent the next
//! keys to the process it had last delivered one to: the minimised window, or
//! the app that had just closed its own.

use crate::focus::hand_off::next_focus;
use crate::focus::{hand_off, press_focus, FocusModel};
use crate::geometry::Rect;
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::z_order::ZStack;

const SHELL: u32 = 5;
const TERMINAL: (u32, u32) = (40, 0x5445_524D);
const EDITOR: (u32, u32) = (41, 0x5445_4458);
const FILES: (u32, u32) = (42, 0x4649_4C45);

struct Desk {
    windows: WindowTable,
    z: ZStack,
    focus: FocusModel,
    /// The pids the compositor was told have focus, in order.
    pushed: Vec<u32>,
}

impl Desk {
    fn new() -> Desk {
        let mut desk = Desk {
            windows: WindowTable::new(),
            z: ZStack::new(),
            focus: FocusModel::new(),
            pushed: Vec::new(),
        };
        // The shell's dock and a toast: popups, the toast opened late, so
        // high in the stack.
        desk.add((SHELL, 0x5442_4152), Kind::Popup);
        desk
    }

    fn add(&mut self, who: (u32, u32), kind: Kind) {
        let window = Window {
            owner_pid: who.0,
            window_id: who.1,
            rect: Rect { x: 0, y: 0, width: 100, height: 100 },
            kind,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
        };
        self.windows.insert(window).expect("room");
    }

    /// window_open: a normal window takes focus on top.
    fn open(&mut self, who: (u32, u32)) {
        self.add(who, Kind::Normal);
        self.focus.set(who.0, who.1);
    }

    fn after(&mut self) {
        if let Some(pid) = hand_off(&self.windows, &mut self.focus) {
            self.pushed.push(pid);
        }
    }

    fn close(&mut self, who: (u32, u32)) {
        self.windows.remove(who.0, who.1).expect("open");
        self.after();
    }

    fn minimize(&mut self, who: (u32, u32)) {
        self.windows.find_mut(who.0, who.1).expect("open").visibility = Visibility::Minimized;
        self.after();
    }

    fn focused(&self) -> Option<(u32, u32)> {
        self.focus.current().map(|f| (f.owner_pid, f.window_id))
    }
}

#[test]
fn closing_the_focused_window_focuses_the_one_under_it() {
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.open(EDITOR);
    desk.add((SHELL, 0x544F_4153), Kind::Popup);
    desk.close(EDITOR);
    assert_eq!(desk.focused(), Some(TERMINAL), "not nobody, and not the shell's toast");
    assert_eq!(desk.pushed, vec![TERMINAL.0], "the compositor hears of it");
}

#[test]
fn minimising_the_focused_window_takes_focus_off_it() {
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.open(EDITOR);
    desk.minimize(EDITOR);
    assert_eq!(desk.focused(), Some(TERMINAL), "the minimised editor takes no more keys");
}

#[test]
fn focus_skips_minimised_windows_and_goes_by_the_stack() {
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.open(FILES);
    desk.open(EDITOR);
    desk.minimize(FILES);
    // The terminal is pressed: raised over the files window it was under.
    press_focus(&mut desk.windows, &mut desk.z, &mut desk.focus, TERMINAL.0, TERMINAL.1)
        .expect("focusable");
    press_focus(&mut desk.windows, &mut desk.z, &mut desk.focus, EDITOR.0, EDITOR.1)
        .expect("focusable");
    desk.close(EDITOR);
    assert_eq!(desk.focused(), Some(TERMINAL), "the files window is minimised");
}

#[test]
fn the_last_window_going_leaves_no_focus() {
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.close(TERMINAL);
    assert_eq!(desk.focused(), None, "the shell's dock is never handed focus");
    assert_eq!(desk.pushed, vec![0]);
}

#[test]
fn a_window_going_that_had_no_focus_changes_nothing() {
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.open(EDITOR);
    desk.close(TERMINAL);
    assert_eq!(desk.focused(), Some(EDITOR));
    assert!(desk.pushed.is_empty(), "nothing to tell the compositor");
}

#[test]
fn instances_of_one_app_are_told_apart_by_pid() {
    // Every terminal opens a window with the terminal's one window id.
    let second = (43, TERMINAL.1);
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.open(second);
    desk.close(second);
    assert_eq!(desk.focused(), Some(TERMINAL));
    assert!(desk.windows.find(second.0, second.1).is_none());
}

#[test]
fn a_dialog_can_be_handed_focus_a_tooltip_cannot() {
    let mut desk = Desk::new();
    desk.open(TERMINAL);
    desk.add((50, 1), Kind::Dialog);
    desk.add((51, 1), Kind::Tooltip);
    desk.open(EDITOR);
    desk.close(EDITOR);
    assert_eq!(desk.focused(), Some((50, 1)));
}

/// Random desks: after any close or minimise, focus is on the highest visible
/// normal or dialog window, or on nothing when none is left.
#[test]
fn focus_always_lands_on_the_top_window_still_showing() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut below = |n: u32| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n as u64) as u32
    };
    for _ in 0..300 {
        let mut desk = Desk::new();
        let mut open: Vec<(u32, u32)> = Vec::new();
        for step in 0..30 {
            match below(4) {
                0 | 1 => {
                    let who = (100 + step, 1);
                    desk.open(who);
                    open.push(who);
                }
                2 if !open.is_empty() => {
                    let who = open.remove(below(open.len() as u32) as usize);
                    desk.close(who);
                }
                3 if !open.is_empty() => {
                    let who = open[below(open.len() as u32) as usize];
                    desk.minimize(who);
                }
                _ => continue,
            }
            let showing = open
                .iter()
                .filter(|w| desk.windows.find(w.0, w.1).unwrap().visibility == Visibility::Visible)
                .max_by_key(|w| desk.windows.find(w.0, w.1).unwrap().z)
                .copied();
            if let Some(f) = desk.focused() {
                let w = desk.windows.find(f.0, f.1).expect("focus is on a window in the table");
                assert!(w.visibility == Visibility::Visible, "focus is on a window on screen");
            } else {
                assert_eq!(showing, None, "a window is showing, so one has focus");
            }
            assert_eq!(next_focus(&desk.windows), showing);
        }
    }
}
