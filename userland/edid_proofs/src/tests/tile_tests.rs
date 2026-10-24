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

//! DisplayID tiled display topology.

use super::displayid_tests::panel;
use crate::displayid::tiled::Tile;
use crate::edid::Edid;

#[test]
fn the_tile_says_where_this_output_sits() {
    let raw = panel();
    let t = Edid::parse(&raw).unwrap().tile().unwrap();
    assert_eq!((t.h_tiles, t.v_tiles, t.h_location, t.v_location), (2, 1, 1, 0));
    assert_eq!((t.width, t.height), (2560, 2880));
    assert!(t.one_enclosure);
    assert_eq!((&t.vendor, t.product, t.serial), (b"DEL", 0x4123, 7));
}

#[test]
fn a_tile_placed_outside_its_grid_is_refused() {
    let mut tile = [0u8; 22];
    tile[2] = 0x10; // column 1 of a one column grid
    assert!(Tile::decode(&tile).is_none());
}
