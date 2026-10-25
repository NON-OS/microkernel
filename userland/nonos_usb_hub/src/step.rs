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

//! What bringing up a hub port asks of the caller at one look.

use crate::error::HubError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Wait,
    /// Ask the hub to reset the port.
    Reset,
    /// The reset is over: clear its change bits, connection included, so
    /// the next look does not take the device for a new one; then the
    /// error, if it left the port disabled.
    ResetOver(Option<HubError>),
    /// Address the device, at this speed, and classify it.
    Address(u8),
    Failed(HubError),
}
