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

//! What setting a pad's reporting up found, for its recovery to go by
//! (drive/reset.rs).

#[derive(Default)]
pub struct Configured {
    /// The Input Mode field read back as something other than touchpad mode
    /// and was written: the pad had lost its mode, as a reset leaves it.
    pub mode_lost: bool,
    /// The Input Mode field did not read back: the pad answered GET_REPORT
    /// with nothing, and its report was built from the descriptor.
    pub mode_unread: bool,
    /// The layout leaves the Input Mode nothing to write: no command or
    /// data register, or a report too long to build. A write the bus
    /// refused is not this; it is tried again.
    pub mode_unwritable: bool,
    /// A write the pad or the bus refused: the set-up did not take.
    pub refused: bool,
}
