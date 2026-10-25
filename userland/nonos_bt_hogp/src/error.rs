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

//! Why the HID host stopped.

use nonos_bt_wire::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HogpError {
    /// A PDU from the peripheral was malformed.
    Wire(WireError),
    /// The peripheral answered with this ATT error.
    Att(u8),
    /// The peripheral wants a link with more security first (Vol 3,
    /// Part F, 3.4.1.1): pair, encrypt, and start again.
    Security(u8),
    /// A response that answers no request sent, or handles that do not
    /// move forward through the database.
    Protocol,
    /// No response within the ATT transaction timeout.
    Timeout,
    /// More services, characteristics or requests than this host bounds.
    TooMany,
    /// No HID service with a boot keyboard or boot mouse input report.
    NoHid,
    /// A boot input report with no client characteristic configuration
    /// descriptor, so its notifications cannot be turned on.
    NoCccd,
}

impl From<WireError> for HogpError {
    fn from(e: WireError) -> Self {
        Self::Wire(e)
    }
}
