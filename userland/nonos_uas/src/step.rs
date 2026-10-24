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
//! What a UAS transport does next in a command's exchange.

/// What the transport does next.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Move the command's data in on the data-in pipe.
    DataIn,
    /// Move it out on the data-out pipe.
    DataOut,
    /// Read the status pipe.
    Status,
    /// Ended; the command was told its status and sense.
    Done,
    /// Refused: a Response IU's code, or 02h for an IU out of turn.
    Failed(u8),
}
