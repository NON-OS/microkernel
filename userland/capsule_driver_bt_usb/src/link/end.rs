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

//! How a link ended.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    /// The controller reported the link down, with this reason (Vol 1,
    /// Part F, 2), as Disconnection Complete gives it.
    Lost(u8),
    /// This host gave up on the link, for this reason; the caller ends it.
    GaveUp(&'static str),
}
