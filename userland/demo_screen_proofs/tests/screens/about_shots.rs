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

//! About on every mode and driver, every section: the window wholly on the
//! screen and between the menubar and the dock, each section painted, and
//! the Display tile's mode inside its ring, never across it.

use nonos_app_skeleton::runner::chrome::{dock_for, menubar_for};
use nonos_app_skeleton::{App, InputEvent, InputKind, KEY_TAB};
use nonos_libc::show;

use super::frame::shoot;
use super::modes::{tag, DRIVERS, MODES};
use super::png::save;
use super::ring::whole_per_mille;
use demo_screen_proofs::about::About;

const SECTIONS: [&str; 7] =
    ["overview", "proofs", "system", "trust", "verify", "display", "licenses"];

fn tab() -> InputEvent {
    InputEvent {
        kind: InputKind::KeyDown,
        flags: 0,
        code: KEY_TAB,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    }
}

#[test]
fn about_opens_whole_on_every_demo_mode_and_paints_every_section() {
    for driver in DRIVERS {
        for (w, h) in MODES {
            show(w, h, driver);
            let mut about = About::new();
            for name in SECTIONS {
                let shot = shoot(&mut about, w, h);
                let at = shot.at;
                assert!(at.initial_x + at.width <= w, "{w}x{h}: off the right");
                assert!(at.initial_y >= menubar_for(w, h), "{w}x{h}: under the menubar");
                assert!(at.initial_y + at.height <= h - dock_for(w, h), "{w}x{h}: under the dock");
                save(&format!("about-{}-{w}x{h}-{name}", tag(driver)), &shot.pixels, at.width);
                if name == "overview" {
                    let whole = whole_per_mille(&shot.pixels, at.width).expect("the display ring");
                    assert!(
                        whole >= 995,
                        "{w}x{h}: the mode is written across its ring ({whole}/1000)"
                    );
                }
                about.on_event(tab());
            }
        }
    }
}
