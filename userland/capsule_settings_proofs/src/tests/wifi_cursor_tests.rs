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

use crate::wifi_cursor::kept_in;

/* One adapter, five networks and two saved ones: a selection on the fourth
 * network stays there; it was put back on the first, kept to one adapter. */
#[test]
fn the_selection_stays_on_its_network_with_one_adapter() {
    assert_eq!(kept_in(3, 5 + 2), 3);
    assert_eq!(kept_in(6, 5 + 2), 6, "the last saved row");
}

#[test]
fn a_selection_past_a_shorter_list_comes_to_its_end() {
    assert_eq!(kept_in(6, 4), 3);
    assert_eq!(kept_in(9, 0), 0, "an empty list leaves it at the top");
}
