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

//! What takes a window down, and what does not.

use crate::window_life::{ends_connection, takes_down, End, Shown};

const SHOWN: Option<Shown> = Some(Shown { surface: 8, toplevel: Some(10) });

#[test]
fn the_toplevel_or_surface_of_the_window_takes_it_down_and_no_other() {
    assert!(takes_down(End::Toplevel(10), SHOWN));
    assert!(takes_down(End::Surface(8), SHOWN));
    assert!(!takes_down(End::Toplevel(11), SHOWN));
    assert!(!takes_down(End::Surface(9), SHOWN));
    assert!(!takes_down(End::Toplevel(10), None), "nothing shown");
}

#[test]
fn the_last_display_descriptor_closes_the_connection() {
    // stdin, stdout, stderr, then the display socket at 3.
    assert!(ends_connection(&[false, false, false, true], 3));
    assert!(!ends_connection(&[false, false, false, true, true], 3), "a dup keeps it open");
    assert!(!ends_connection(&[false, false, false, true], 2), "not the display");
    assert!(!ends_connection(&[false, false, false, true], 7), "no such descriptor");
}
