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

//! The host wire status a failed client call stands for, as the hub class
//! reads it: E_INVAL marks an op the controller driver does not serve.

use nonos_usb_hub::E_BUSY;

use crate::xhci::XhciClientError;

/// The host wire's I/O error, for a reply that never came.
const E_IO: i32 = -5;

pub(super) fn status(e: XhciClientError) -> i32 {
    match e {
        XhciClientError::Status(s) => s,
        XhciClientError::Busy => E_BUSY,
        _ => E_IO,
    }
}
