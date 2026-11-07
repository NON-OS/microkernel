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

//! The window a receiver's view took goes back only when the receiver does
//! not own the surface and no page of the window maps anything any more.

use super::view_rule::{window_back, Window};

#[test]
fn a_view_down_whole_gives_its_window_back() {
    assert_eq!(window_back(false, 2025, 0), Window::Free(2025));
}

#[test]
fn an_owner_record_is_its_own_mapping_and_stays() {
    assert_eq!(window_back(true, 2025, 0), Window::Kept);
}

#[test]
fn a_window_a_page_of_which_still_maps_is_kept() {
    for still in [1, 2, 2025] {
        assert_eq!(window_back(false, 2025, still), Window::Kept);
    }
}

#[test]
fn a_view_of_no_frames_gives_nothing_back() {
    assert_eq!(window_back(false, 0, 0), Window::Kept);
}
