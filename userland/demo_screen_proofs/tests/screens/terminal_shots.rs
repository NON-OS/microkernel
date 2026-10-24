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

//! The terminal on every mode and driver: the window wholly on the screen
//! between the menubar and the dock, painted as it first opens and after
//! `help`, the longest thing the shell prints on its own.

use nonos_app_skeleton::runner::chrome::{dock_for, menubar_for};
use nonos_app_skeleton::{App, InputEvent, InputKind, KEY_ENTER};
use nonos_libc::show;

use super::frame::shoot;
use super::modes::{tag, DRIVERS, MODES};
use super::png::save;
use demo_screen_proofs::term::Terminal;

#[test]
fn the_terminal_opens_whole_on_every_demo_mode() {
    for driver in DRIVERS {
        for (w, h) in MODES {
            show(w, h, driver);
            let mut term = Terminal::new();
            let shot = shoot(&mut term, w, h);
            let at = shot.at;
            assert!(at.initial_x + at.width <= w, "{w}x{h}: off the right");
            assert!(at.initial_y >= menubar_for(w, h), "{w}x{h}: under the menubar");
            assert!(at.initial_y + at.height <= h - dock_for(w, h), "{w}x{h}: under the dock");
            save(&format!("terminal-{}-{w}x{h}-open", tag(driver)), &shot.pixels, at.width);
            run(&mut term, b"help");
            let shot = shoot(&mut term, w, h);
            save(&format!("terminal-{}-{w}x{h}-help", tag(driver)), &shot.pixels, at.width);
        }
    }
}

fn key(code: u32) -> InputEvent {
    InputEvent {
        kind: InputKind::KeyDown,
        flags: 0,
        code,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    }
}

/// Type `line` and press Enter, then let the terminal tick as the runner would.
fn run(term: &mut Terminal, line: &[u8]) {
    line.iter().for_each(|&c| _ = term.on_event(key(c as u32)));
    term.on_event(key(KEY_ENTER));
    (0..4).for_each(|_| _ = term.on_tick());
}
