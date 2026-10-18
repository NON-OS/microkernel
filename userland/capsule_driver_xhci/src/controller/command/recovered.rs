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
/// The endpoint was not in the state the command needs.
pub const CC_CONTEXT_STATE_ERROR: u8 = 19;

/// How the endpoint was brought back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovered {
    /// It was Halted, and Reset Endpoint also reset its data toggle or
    /// sequence number.
    Reset,
    /// It was Running or already Stopped: Stop Endpoint left the toggle
    /// where it was.
    Stopped,
}
