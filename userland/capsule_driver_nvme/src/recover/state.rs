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

//! The named states recovery ends in. Pure, so the host proofs hold them.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovered {
    /// The command came back after an Abort, aborted or done before the
    /// Abort reached it; the queue goes on.
    Aborted,
    /// The controller was disabled, which drops every command it held, and
    /// enabled again with its queues rebuilt in the same memory.
    Reset,
    /// CSTS read all ones: nothing answers at BAR0, as when the drive is
    /// pulled. No reset brings it back.
    Gone,
    /// The reset did not complete. The controller may still own the data
    /// buffer, so no request touches it again.
    Failed,
}

impl Recovered {
    /// Whether the I/O queue serves requests again.
    pub const fn serves(self) -> bool {
        matches!(self, Recovered::Aborted | Recovered::Reset)
    }

    /// The words the console line ends with.
    pub const fn words(self) -> &'static str {
        match self {
            Recovered::Aborted => "it came back after an Abort",
            Recovered::Reset => "controller reset, queues rebuilt in place",
            Recovered::Gone => "controller gone from the bus, no more I/O",
            Recovered::Failed => "reset failed, no more I/O",
        }
    }
}
