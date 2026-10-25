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

//! The Command register flags each SD command the driver sends must carry.

/// The flags each SD command needs (4.9): R1, R6 and R7 index and CRC
/// checked, R2 CRC only, R3 neither; the reads and writes carry data, and
/// CMD12 is an abort with busy.
pub(super) const FLAGS: &[(u8, u16)] = &[
    (0, 0x00),
    (2, 0x09),
    (3, 0x1a),
    (7, 0x1a),
    (8, 0x1a),
    (9, 0x09),
    (12, 0xdb),
    (13, 0x1a),
    (16, 0x1a),
    (17, 0x3a),
    (18, 0x3a),
    (24, 0x3a),
    (25, 0x3a),
    (55, 0x1a),
];
