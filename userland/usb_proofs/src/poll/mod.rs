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

//! The interrupt read's length rule, beside the report buffer size it is
//! bounded by. The poll loop around them makes system calls and stays out.

// RESCAN_INTERVAL belongs to the poll loop, which is not included.
#[allow(dead_code)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/constants.rs"]
mod constants;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/read_len.rs"]
pub mod read_len;
