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
//! The resize arrows along an edge: left and right, top and bottom.

use super::cursor_shapes::Shape;

/// Left and right edges.
pub const EW: Shape = Shape {
    rows: &[
        b"    XXX       XXX",
        b"   XXoX       XoXX",
        b"  XXooX       XooXX",
        b"XXXoooXXXXXXXXXoooXXX",
        b"XoooooooooooooooooooX",
        b"XXXoooXXXXXXXXXoooXXX",
        b"  XXooX       XooXX",
        b"   XXoX       XoXX",
        b"    XXX       XXX",
    ],
    hot: (10, 4),
};

/// Top and bottom edges.
pub const NS: Shape = Shape {
    rows: &[
        b"   XXX",
        b"   XoX",
        b"  XXoXX",
        b" XXoooXX",
        b"XXoooooXX",
        b"XoooooooX",
        b"XXXXoXXXX",
        b"   XoX",
        b"   XoX",
        b"   XoX",
        b"   XoX",
        b"   XoX",
        b"   XoX",
        b"   XoX",
        b"XXXXoXXXX",
        b"XoooooooX",
        b"XXoooooXX",
        b" XXoooXX",
        b"  XXoXX",
        b"   XoX",
        b"   XXX",
    ],
    hot: (4, 10),
};
