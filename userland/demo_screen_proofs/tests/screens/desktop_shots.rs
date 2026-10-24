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

//! Whole screens, each demo mode and driver, About or the Terminal the first
//! window, placed by the window manager's rule, clear of the shell's chrome.

use demo_screen_proofs::about::About;
use demo_screen_proofs::desk;
use demo_screen_proofs::term::Terminal;
use nonos_libc::show;

use super::compose::screen;
use super::frame::{shoot, Shot};
use super::modes::{tag, DRIVERS, MODES};
use super::png::save;
use super::wallpaper_mount::wallpaper;

fn check_and_save(name: &str, w: u32, h: u32, shot: &Shot, d: &desk::Desk, wall: &[u32]) {
    let (px, r) = screen(w, h, [wall, &d.desk, &d.chrome], shot);
    let drawn =
        |y0: u32, y1: u32| (y0 * w..y1 * w).filter(|&i| d.chrome[i as usize] >> 24 != 0).count();
    assert!(drawn(0, 20) > (w as usize) * 10, "{name}: no menubar");
    assert!(drawn(h - 60, h) > 1000, "{name}: no dock");
    for y in r.y..(r.y + r.height).min(h) {
        for x in r.x..(r.x + r.width).min(w) {
            assert_eq!(
                d.chrome[(y * w + x) as usize] >> 24,
                0,
                "{name}: chrome over the window at {x},{y}"
            );
        }
    }
    save(name, &px, w);
}

#[test]
fn every_demo_screen_composes_with_the_window_clear_of_the_chrome() {
    for driver in DRIVERS {
        for (w, h) in MODES {
            show(w, h, driver);
            let (d, wall) = (desk::paint(w, h), wallpaper(w, h));
            let about = shoot(&mut About::new(), w, h);
            check_and_save(
                &format!("screen-{}-{w}x{h}-about", tag(driver)),
                w,
                h,
                &about,
                &d,
                &wall,
            );
            let term = shoot(&mut Terminal::new(), w, h);
            check_and_save(
                &format!("screen-{}-{w}x{h}-terminal", tag(driver)),
                w,
                h,
                &term,
                &d,
                &wall,
            );
        }
    }
}
