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

use super::{n0, n1, n2, n3, n4, n5, n6, n7};

/// The table in byte order, split across files only to keep each readable.
/// Every row of a chunk sorts before every row of the next one.
pub static CHUNKS: [&[(&str, char, char)]; 8] =
    [n0::ROWS, n1::ROWS, n2::ROWS, n3::ROWS, n4::ROWS, n5::ROWS, n6::ROWS, n7::ROWS];
