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

use super::colors::{bg_color, make_attr};

#[test]
fn a_bright_background_never_sets_the_blink_bit() {
    for bg in 0..=u8::MAX {
        assert_eq!(make_attr(15, bg) & 0x80, 0, "background {bg}");
    }
    assert_eq!(bg_color(0x8F), 0);
    assert_eq!(bg_color(make_attr(15, 5)), 5);
}
