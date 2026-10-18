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

//! The reply status: zero, or a negative errno saying why.

pub const E_OK: i32 = 0;
/// The card or the reader failed the request.
pub const E_IO: i32 = -5;
/// The range runs past the card's last block.
pub const E_NXIO: i32 = -6;
/// The sender may not reach the medium (serve/admits.rs).
pub const E_ACCES: i32 = -13;
/// Service has ended: another card went in where the served one was, or
/// the card there could not be identified (Medium::Changed).
pub const E_NODEV: i32 = -19;
/// An op this service does not have, a body where none belongs, or a
/// block count of zero or past MAX_RW_BLOCKS.
pub const E_INVAL: i32 = -22;
/// The card's write-protect switch is set.
pub const E_ROFS: i32 = -30;
/// A body whose length is not what the header and the op say.
pub const E_MSGSIZE: i32 = -90;
/// The card did not finish in its time; it is identified again before it
/// serves another request.
pub const E_TIMEDOUT: i32 = -110;
/// No card to serve: none in the slot yet, or the served one is out.
pub const E_NOMEDIUM: i32 = -123;
