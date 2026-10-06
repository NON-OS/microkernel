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

//! What the wallpaper service says about the wallpaper chosen: a fetch that
//! stops says how far it came, once; the wallpaper going on the desktop is
//! said every time, and a failure after it is said again. Each test uses
//! wallpapers of its own, since which failures were said is the service's
//! one record for every thread.

use nonos_libc::take_said;

use crate::wallpaper_say::{failed, shown, stopped};

fn said() -> Vec<String> {
    take_said().into_iter().map(|l| String::from_utf8(l).unwrap()).collect()
}

#[test]
fn a_fetch_that_stops_says_how_far_it_came_once() {
    let _ = said();
    stopped(62, 20480, 101704);
    stopped(62, 40960, 101704);
    failed(62, "decoding it");
    assert_eq!(
        said(),
        ["[WALLPAPER] wallpaper 62: fetching it from the catalog stopped at byte 20480 of 101704\n"]
    );
    stopped(0, 0, 159430);
    assert_eq!(
        said(),
        ["[WALLPAPER] wallpaper 0: fetching it from the catalog stopped at byte 0 of 159430\n"]
    );
}

#[test]
fn a_wallpaper_shown_is_said_and_its_next_failure_is_said_again() {
    let _ = said();
    failed(41, "decoding it");
    failed(41, "decoding it");
    shown(41);
    shown(41);
    failed(41, "painting it");
    assert_eq!(
        said(),
        [
            "[WALLPAPER] wallpaper 41: decoding it failed\n",
            "[WALLPAPER] wallpaper 41 shown\n",
            "[WALLPAPER] wallpaper 41 shown\n",
            "[WALLPAPER] wallpaper 41: painting it failed\n",
        ]
    );
}
