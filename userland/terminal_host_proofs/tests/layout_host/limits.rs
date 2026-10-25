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

//! The width thresholds the solver works to.

use crate::layout::limits::{LEFT_RAIL_MIN_W, LEFT_RAIL_W, MIN_BODY_W};

/// The breakpoints are part of the layout spec, not incidental numbers; a
/// silent edit here changes which chrome appears at which window size.
#[test]
fn the_thresholds_match_the_spec() {
    assert_eq!(MIN_BODY_W, 320);
    assert_eq!(LEFT_RAIL_MIN_W, 640);
    assert_eq!(LEFT_RAIL_W, 320);
    assert_eq!(LEFT_RAIL_MIN_W, LEFT_RAIL_W + MIN_BODY_W);
}
