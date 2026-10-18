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

//! The Command register flags each eMMC command the driver sends must carry.

pub const RESP_FLAGS: &[(u8, u16)] = &[
    (0, 0x00),
    (1, 0x02),
    (2, 0x09),
    (3, 0x1a),
    (6, 0x1b),
    (7, 0x1a),
    (8, 0x3a),
    (9, 0x09),
    (12, 0xdb),
    (13, 0x1a),
    (16, 0x1a),
    (17, 0x3a),
    (18, 0x3a),
    (23, 0x1a),
    (24, 0x3a),
    (25, 0x3a),
];
