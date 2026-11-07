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

use nonos_app_skeleton::runner::swap_paint::whole_needed;
use nonos_app_skeleton::swap::{Damage, Stale};

const CONTENT: (u32, u32, u32, u32) = (4, 30, 152, 86);

fn boxed(x: u32, y: u32, width: u32, height: u32) -> Stale {
    Stale::Box(Damage { x, y, width, height })
}

#[test]
fn an_up_to_date_buffer_draws_the_content_alone() {
    assert!(!whole_needed(Stale::None, CONTENT));
}

#[test]
fn a_buffer_never_drawn_draws_the_whole_window() {
    assert!(whole_needed(Stale::Whole, CONTENT));
}

#[test]
fn stale_content_alone_draws_the_content_alone() {
    assert!(!whole_needed(boxed(4, 30, 152, 86), CONTENT), "exactly the content");
    assert!(!whole_needed(boxed(50, 40, 10, 10), CONTENT), "inside it");
}

#[test]
fn stale_past_the_content_draws_the_whole_window() {
    assert!(whole_needed(boxed(3, 30, 10, 10), CONTENT), "into the left border");
    assert!(whole_needed(boxed(50, 29, 10, 10), CONTENT), "into the title bar");
    assert!(whole_needed(boxed(150, 40, 7, 10), CONTENT), "past the right edge");
    assert!(whole_needed(boxed(50, 110, 10, 7), CONTENT), "past the bottom");
    assert!(whole_needed(boxed(0, 0, 160, 120), CONTENT), "the whole window");
}
