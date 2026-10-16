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

//! What the shell says of a search or a launch that came to nothing.

use super::apps::line;
use crate::says::{
    launchpad_empty, not_opened, NOTHING_REFUSED, NOT_ASKED, NO_WINDOW, OFF_AT_SETUP,
};
use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::expect::EXPECT_MS;

#[test]
fn a_launchpad_search_that_matches_nothing_says_so_and_the_way_back() {
    let lines = launchpad_empty("zzz", 0).expect("an empty grid is never left blank");
    assert!(lines[0].starts_with("No app"));
    assert!(lines[1].contains("Backspace") && lines[1].contains("Esc"));
    assert_eq!(launchpad_empty("term", 1), None, "a match shows its tile, not the line");
    assert_eq!(launchpad_empty("", 0), None, "nothing typed is not a failed search");
}

/// A launch that opened nothing says why, from the kernel's answer to the
/// spawn: it said "did not open" and no more, whatever the kernel answered.
/// Every reason fits whole after every dock name.
#[test]
fn a_launch_that_opened_nothing_says_why() {
    assert_eq!(not_opened(NOT_ASKED), b" is not running; it has one window");
    assert_eq!(not_opened(-2), not_opened(NOT_ASKED), "an app with one window only");
    assert_eq!(not_opened(-13), OFF_AT_SETUP);
    assert_eq!(not_opened(-16), b" did not open: busy, try again");
    assert_eq!(not_opened(-1), b" did not open: not permitted");
    assert_eq!(not_opened(NOTHING_REFUSED), b" did not open: its window is gone");
    assert_eq!(not_opened(-99), b" did not open: the kernel refused");
    for app in LAUNCHER_APPS.iter() {
        for rc in [NOT_ASKED, NOTHING_REFUSED, -1, -2, -13, -16, -22, -99] {
            let said = line(app.label, not_opened(rc));
            assert!(
                said.starts_with(app.label),
                "{} cut by {rc}",
                String::from_utf8_lossy(app.label)
            );
            assert!(said.ends_with(not_opened(rc)));
        }
    }
}

/// The wait the toast names is the wait the shell keeps.
#[test]
fn the_wait_said_is_the_wait_kept() {
    let said = format!(" did not open: no window in {} s", EXPECT_MS / 1000);
    assert_eq!(NO_WINDOW, said.as_bytes());
    for app in LAUNCHER_APPS.iter() {
        assert!(line(app.label, NO_WINDOW).starts_with(app.label));
    }
}
