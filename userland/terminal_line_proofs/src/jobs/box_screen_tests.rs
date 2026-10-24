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

//! The report on a screen a hostile guest left behind: whatever modes it set
//! last, the report drawn after SCREEN_RESET is the last thing on the
//! screen, every line of it, character for character. The screen is the
//! terminal's own VT, driven as the terminal drives it (a line, then "\n"
//! taken as "\r\n").

use nonos_vt::term::Term;

use super::box_screen::SCREEN_RESET;
use super::box_screen_left::LEFT;

const REPORT: [&str; 4] =
    ["BOX REPORT v1", "  refused  2", "    write         1  /home/b", "  exit     0"];

fn drawn(left: &str) -> Term {
    let mut t = Term::new(80, 24, 200);
    t.feed(b"some output\r\n");
    t.feed(left.as_bytes());
    t.feed(SCREEN_RESET);
    for line in REPORT {
        t.feed(line.as_bytes());
        t.feed(b"\r\n");
    }
    t
}

#[test]
fn whatever_the_guest_left_the_report_is_drawn_last_and_whole() {
    for left in LEFT.iter().copied().chain([LEFT.concat().as_str()]) {
        let t = drawn(left);
        let rows = t.screen_text();
        let foot: Vec<&str> = rows.iter().map(String::as_str).rev().skip(1).take(4).collect();
        let want: Vec<&str> = REPORT.iter().rev().copied().collect();
        assert_eq!(foot, want, "after {left:?}: {rows:#?}");
        let m = &t.modes;
        assert!(!t.alt_active() && !m.newline && !m.reverse_video, "after {left:?}");
        assert!(!m.insert && !m.origin && m.autowrap, "after {left:?}");
    }
}
