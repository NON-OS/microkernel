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

//! Kernel-side client for the USB mass-storage driver capsule,
//! driver.usb_msc0, which moves BOT/SCSI traffic over driver.xhci0. The
//! capsule's embed, spawn and liveness live in
//! `userspace::capsule_driver_usb_msc`; this is the block surface the
//! kernel's block layer reads and writes through.

mod capability;
mod error;
mod io;
mod protocol;
mod transport;

pub use error::DriverUsbMscError;
pub use io::{capacity, flush, geometry, read_blocks, write_blocks};
pub(crate) use transport::REPLY_INBOX;
