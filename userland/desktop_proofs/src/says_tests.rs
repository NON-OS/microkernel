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

//! The desktop says what did not happen, in words that fit a toast, and the
//! menu bar claims no battery state it was never given: a reading is a
//! percent, no battery is "No battery", and an unreadable one says so.

use crate::battery_text::{label, Battery, LABEL_MAX};
use crate::says::{
    launchpad_empty, named, not_opened, package, vfs, NOTHING_REFUSED, NOT_ASKED, NO_REPLY,
    NO_WINDOW, OFF_AT_SETUP,
};
use crate::shell_apps::LAUNCHER_APPS;
use crate::taskbar::expect::EXPECT_MS;

/// What a toast holds (`state/toast.rs`).
const TOAST: usize = 48;

fn line(head: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut out = [0u8; TOAST];
    let n = named(head, tail, &mut out);
    out[..n].to_vec()
}

#[test]
fn an_app_that_did_not_open_is_named() {
    assert_eq!(line(b"Terminal", not_opened(-16)), b"Terminal did not open: busy, try again");
    assert_eq!(line(b"Browser", OFF_AT_SETUP), b"Browser turned off at setup");
}

#[test]
fn a_long_name_is_cut_and_the_reason_is_kept_whole() {
    let long = [b'x'; 64];
    let said = line(&long, not_opened(-16));
    assert_eq!(said.len(), TOAST);
    assert!(said.ends_with(not_opened(-16)));
}

#[test]
fn a_refused_package_is_said_in_words_that_fit_a_toast() {
    for code in [-2, -5, -11, -13, -17, -22, -71, -99] {
        let said = line(b"Package: ", package(code));
        assert!(said.starts_with(b"Package: "));
        assert!(said.ends_with(package(code)), "code {code} was cut");
        assert!(!said.iter().any(u8::is_ascii_digit), "code {code} shows a number");
    }
    assert_eq!(package(-13), b"signature or digest failed verification");
    assert_eq!(package(-17), b"already installed");
}

#[test]
fn a_refused_desktop_action_says_why() {
    assert_eq!(line(b"Could not delete: ", vfs(-39)), b"Could not delete: the folder is not empty");
    assert_eq!(line(b"Could not rename: ", vfs(-17)), b"Could not rename: that name is taken");
    assert_eq!(vfs(NO_REPLY), b"the file service did not answer");
    assert_eq!(vfs(-28), b"no space left");
    assert_eq!(vfs(-5), b"the file service refused");
}

#[test]
fn a_desktop_with_no_battery_says_no_battery() {
    let mut buf = [0u8; LABEL_MAX];
    let b = Battery::from_status(-19);
    assert_eq!(b, Battery::None);
    let n = label(b, &mut buf);
    assert_eq!(&buf[..n], b"No battery");
}

#[test]
fn an_unreadable_battery_says_so_in_words_not_zero_percent() {
    for rc in [-95, -5, -1, 101, i64::MIN] {
        let b = Battery::from_status(rc);
        assert_eq!(b, Battery::Unavailable, "rc {rc}");
        assert_eq!(b.percent(), None);
        let mut buf = [0u8; LABEL_MAX];
        let n = label(b, &mut buf);
        assert_eq!(&buf[..n], b"Battery status unavailable");
        assert!(!buf[..n].contains(&b'%'));
    }
}

#[test]
fn a_battery_reading_is_drawn_as_a_percent() {
    let mut buf = [0u8; LABEL_MAX];
    let n = label(Battery::from_status(57), &mut buf);
    assert_eq!(&buf[..n], b"57%");
    let n = label(Battery::from_status(7), &mut buf);
    assert_eq!(&buf[..n], b"7%");
    let n = label(Battery::from_status(100), &mut buf);
    assert_eq!(&buf[..n], b"100%");
    let n = label(Battery::from_status(0), &mut buf);
    assert_eq!(&buf[..n], b"0%", "a real 0 reading is still a reading");
}

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
