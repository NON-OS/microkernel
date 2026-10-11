// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! A USB stick (driver.usb_msc0). The loader's copy of a live stick
//! answers for it, and the driver's search, which it cannot log itself, is
//! said beside its line.

use core::sync::atomic::AtomicU64;

use super::super::disk::Disk;
use super::super::map_usb_msc::map_usb_msc_error;
use super::usb_msc_fit::sectors_fit;
use crate::hardware::usb_msc_capsule as c;

static SAID: AtomicU64 = AtomicU64::new(0);

pub(in super::super) static USB_MSC: Disk = Disk {
    shown: |_| "USB stick (driver.usb_msc0)",
    announced: |_| "USB mass storage (driver.usb_msc0)",
    capacity: |_| c::capacity().map_err(map_usb_msc_error),
    read: |_, lba, out| c::read_blocks(lba, out).map_err(map_usb_msc_error),
    write: |_, lba, data| c::write_blocks(lba, data).map_err(map_usb_msc_error),
    flush: |_| c::flush().map_err(map_usb_msc_error),
    fits: |_| sectors_fit(),
    said: |_| &SAID,
    copy_answers: true,
    report: Some(c::report_line),
};
