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

//! The modes of KDE's server-decoration.xml (version 1), and the one a
//! window is given for the one its client asks. Pure, so the host proofs
//! hold it.

pub const NONE: u32 = 0;
pub const CLIENT: u32 = 1;
pub const SERVER: u32 = 2;

/// Whether the client draws its own frame, and the mode it is told, for a
/// request of `asked`: Client is let draw its own, as xdg-decoration's
/// client side is; None and Server, and any other, get the NØNOS frame.
pub fn answer(asked: u32) -> (bool, u32) {
    match asked {
        CLIENT => (true, CLIENT),
        _ => (false, SERVER),
    }
}
