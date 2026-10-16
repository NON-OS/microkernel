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
use crate::constants::{PLS_COMPLIANCE, PLS_INACTIVE, PORTSC_PLS_MASK, PORTSC_PLS_SHIFT};

/// How a port is brought to Enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortAction {
    /// Hot reset through PR.
    Reset,
    /// Warm reset through WPR: a USB 3 link in Inactive or Compliance.
    WarmReset,
}

/// What `portsc` calls for on a USB 3 port (`usb3`) or any other.
pub fn port_action(portsc: u32, usb3: bool) -> PortAction {
    if !usb3 {
        return PortAction::Reset;
    }
    let pls = (portsc & PORTSC_PLS_MASK) >> PORTSC_PLS_SHIFT;
    if pls == PLS_INACTIVE || pls == PLS_COMPLIANCE {
        return PortAction::WarmReset;
    }
    // A USB 3 port whose link trained is enabled already, but the device on it
    // may have been addressed by another class driver that probed it and let
    // it go. Real devices refuse a second Address Device without a reset in
    // between (QEMU's do not), so every port is reset before it is addressed,
    // as Linux hub_port_init does.
    PortAction::Reset
}
