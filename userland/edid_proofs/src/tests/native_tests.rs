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

//! The native mode a panel names first.

use crate::build::{base, put};
use crate::build_dtd::dtd_1080p;
use crate::edid::Edid;

#[test]
fn the_first_detailed_timing_is_native_and_preferred() {
    let mut b = base();
    put(&mut b, 1, &dtd_1080p());
    let e = Edid::parse(&b).unwrap();
    let n = e.native().unwrap();
    assert_eq!((n.width, n.height, n.width_mm, n.height_mm), (1920, 1080, 531, 299));
    assert_eq!(e.preferred().unwrap().h_active, 1920);
    assert_eq!(e.physical_mm(), 531 | 299 << 16);
}
