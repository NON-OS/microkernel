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

//! The Bluetooth page's own pieces: the panel's logic (`panel`), which the
//! proofs run, and its calls to driver.bt_usb0 (`service`), which are
//! syscalls. The wire format is `nonos_bt_proto`, shared with the capsule.

pub mod panel;
mod service;

pub use panel::Panel;
pub use service::{poll, run};
