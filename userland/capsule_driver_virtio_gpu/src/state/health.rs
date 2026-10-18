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

//! The control queue's named state, as OP_HEALTHCHECK reports it
//! (plan/50 C1).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Health {
    /// Commands reach the device. It may have stalled and been recovered in
    /// place since; the driver counts those.
    Live,
    /// The device stalled and no recovery try brought it back. It is left
    /// reset, the claim and every grant are kept (the compositor still
    /// draws into the primary surface's frames), and device ops answer
    /// E_DEVICE.
    Down,
}

impl Health {
    /// The word OP_HEALTHCHECK carries.
    pub const fn code(self) -> u32 {
        match self {
            Self::Live => 0,
            Self::Down => 1,
        }
    }
}
