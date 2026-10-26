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

//! The driver's bound device, each file mounted once at the path the driver
//! names (`crate::disk`) with the driver's own exports; the BOT command and
//! the disk's types are open to the sweep as well.

#[path = "../../../capsule_driver_usb_msc/src/disk/bot.rs"]
pub(crate) mod bot;
#[path = "../../../capsule_driver_usb_msc/src/disk/capacity.rs"]
mod capacity;
#[path = "../../../capsule_driver_usb_msc/src/disk/csw.rs"]
mod csw;
#[path = "../../../capsule_driver_usb_msc/src/disk/data.rs"]
mod data;
#[path = "../../../capsule_driver_usb_msc/src/disk/data_phase.rs"]
mod data_phase;
#[path = "../../../capsule_driver_usb_msc/src/disk/digest.rs"]
mod digest;
#[path = "../../../capsule_driver_usb_msc/src/disk/inquiry.rs"]
mod inquiry;
#[path = "../../../capsule_driver_usb_msc/src/disk/lun.rs"]
mod lun;
#[path = "../../../capsule_driver_usb_msc/src/disk/ready.rs"]
mod ready;
#[path = "../../../capsule_driver_usb_msc/src/disk/recover.rs"]
mod recover;
#[path = "../../../capsule_driver_usb_msc/src/disk/scsi_io.rs"]
mod scsi_io;
#[path = "../../../capsule_driver_usb_msc/src/disk/sense.rs"]
mod sense;
#[path = "../../../capsule_driver_usb_msc/src/disk/serial.rs"]
mod serial;
#[path = "../../../capsule_driver_usb_msc/src/disk/transport.rs"]
mod transport;
#[path = "../../../capsule_driver_usb_msc/src/disk/types.rs"]
pub(crate) mod types;
#[path = "../../../capsule_driver_usb_msc/src/disk/uas.rs"]
mod uas;
#[path = "../../../capsule_driver_usb_msc/src/disk/uas_cmd.rs"]
mod uas_cmd;
#[path = "../../../capsule_driver_usb_msc/src/disk/uas_data.rs"]
mod uas_data;

pub use capacity::{capacity, sync_cache};
pub use inquiry::inquiry;
pub use lun::max_lun;
pub use ready::{test_ready, Unit};
pub use scsi_io::{read, write};
pub use serial::serial;
pub use types::Disk;
