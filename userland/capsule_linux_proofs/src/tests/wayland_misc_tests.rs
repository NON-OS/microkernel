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

//! xdg_activation_v1 and wp_single_pixel_buffer_manager_v1, against
//! xdg-activation-v1.xml and single-pixel-buffer-v1.xml.

use crate::ops::misc::*;

#[test]
fn activation_numbers_are_the_protocols() {
    let activation = (ACTIVATION_DESTROY, ACTIVATION_GET_TOKEN, ACTIVATION_ACTIVATE);
    assert_eq!(activation, (0, 1, 2));
    // set_serial, set_app_id, set_surface, commit, destroy; event done.
    assert_eq!((TOKEN_SET_FIRST, TOKEN_SET_LAST, TOKEN_COMMIT, TOKEN_DESTROY), (0, 2, 3, 4));
    assert_eq!(TOKEN_DONE, 0);
}

#[test]
fn single_pixel_numbers_are_the_protocols() {
    assert_eq!((SINGLE_PIXEL_DESTROY, SINGLE_PIXEL_CREATE_BUFFER), (0, 1));
}
