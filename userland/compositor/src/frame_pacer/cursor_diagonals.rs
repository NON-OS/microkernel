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
//! The resize arrows at a corner.

use super::cursor_shapes::Shape;

/// The top-left and bottom-right corners.
pub const NWSE: Shape = Shape {
    rows: &[
        b"XXXXXXX",
        b"XoooooX",
        b"XooooXX",
        b"XoooXX",
        b"XooXoXX",
        b"XoXXXoXX",
        b"XXX XXoXX",
        b"     XXoXX",
        b"      XXoXX XXX",
        b"       XXoXXXoX",
        b"        XXoXooX",
        b"         XXoooX",
        b"        XXooooX",
        b"        XoooooX",
        b"        XXXXXXX",
    ],
    hot: (7, 7),
};

/// The top-right and bottom-left corners.
pub const NESW: Shape = Shape {
    rows: &[
        b"        XXXXXXX",
        b"        XoooooX",
        b"        XXooooX",
        b"         XXoooX",
        b"        XXoXooX",
        b"       XXoXXXoX",
        b"      XXoXX XXX",
        b"     XXoXX",
        b"XXX XXoXX",
        b"XoXXXoXX",
        b"XooXoXX",
        b"XoooXX",
        b"XooooXX",
        b"XoooooX",
        b"XXXXXXX",
    ],
    hot: (7, 7),
};
