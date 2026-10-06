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

//! Where bringing up a hub or a device behind one stopped, each with the
//! words the log says it in.

use crate::xhci::XhciClientError;

/// E_INVAL, which the controller driver answers to an op it does not know.
const E_INVAL: i32 = -22;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubError {
    Descriptor,
    Depth,
    PortStatus,
    ResetRefused,
    ResetTimeout,
    Gone,
    NotEnabled,
    TooDeep,
    NoSlot,
    Address(XhciClientError),
}

impl HubError {
    pub fn says(self) -> &'static [u8] {
        match self {
            Self::Descriptor => b"the hub descriptor could not be read",
            Self::Depth => b"the SuperSpeed hub refused SET_HUB_DEPTH",
            Self::PortStatus => b"the port status could not be read",
            Self::ResetRefused => b"the hub refused the port reset",
            Self::ResetTimeout => b"the port reset did not finish within 800 ms",
            Self::Gone => b"the device left during the port reset",
            Self::NotEnabled => b"the port was not enabled after its reset",
            Self::TooDeep => b"more than five hubs deep, which USB does not allow",
            Self::NoSlot => b"the controller has no free slot",
            Self::Address(XhciClientError::Status(E_INVAL)) => {
                b"the controller driver cannot address a device behind a hub yet (op 0x20 refused)"
            }
            Self::Address(XhciClientError::Busy) => b"the controller driver said the port is busy",
            Self::Address(_) => b"Address Device failed",
        }
    }
}
