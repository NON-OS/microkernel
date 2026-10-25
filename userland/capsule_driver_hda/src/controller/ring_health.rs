// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The verb transport's health (ring_recover.rs).

/// How the verb transport is holding up.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Health {
    /// Verbs the controller never fetched since one was last fetched or
    /// answered, or the rings were last reset.
    pub unfetched: u8,
    /// Ring resets since any verb was last answered.
    pub resets_unanswered: u8,
    /// Ring resets in all.
    pub resets: u32,
    /// Responses the RIRB dropped for want of room.
    pub overruns: u32,
    /// The rings were given up for the Immediate Command interface.
    pub immediate: bool,
}
