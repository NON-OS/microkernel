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

//! A driver's reply status in words. Besides the errnos, the drivers send
//! what the device itself said when it ended a command with an error, so a
//! failed install names the cause and not only a number:
//!
//! - NVMe: -(0x1000 | SCT << 8 | SC), the completion's status;
//! - SATA: -(0x1_0000 | ERR << 8 | STS), the disk's task file;
//! - eMMC host: -(0x100_0000 | CMD << 16 | Error Interrupt Status);
//! - eMMC card: -(0x200_0000 | CMD << 16 | the R1 error bit's number).
//!
//! Pure, for the host proofs.

use alloc::format;
use alloc::string::String;

// By path, so a proof crate that takes this file by path takes them too.
#[path = "status_text/disk.rs"]
mod disk;
#[path = "status_text/emmc.rs"]
mod emmc;

use disk::{ata, nvme};
use emmc::{emmc_card, emmc_host};

pub fn describe(status: i32) -> String {
    let code = -(status as i64);
    match code {
        5 => String::from("I/O error"),
        6 => String::from("past the end of the disk"),
        13 => String::from("the driver does not serve this program"),
        19 => String::from("no disk behind the driver"),
        22 => String::from("a request the driver refused"),
        30 => String::from("the disk is write-protected"),
        90 => String::from("a request of the wrong size"),
        110 => String::from("the device did not answer in time"),
        123 => String::from("no medium in the drive"),
        0x1000..=0x17ff => nvme(((code >> 8) & 0x7) as u8, (code & 0xff) as u8),
        0x1_0000..=0x1_ffff => ata(((code >> 8) & 0xff) as u8, (code & 0xff) as u8),
        0x100_0000..=0x1ff_ffff => emmc_host(((code >> 16) & 0x3f) as u8, (code & 0xffff) as u16),
        0x200_0000..=0x2ff_ffff => emmc_card(((code >> 16) & 0x3f) as u8, (code & 0x1f) as u8),
        _ => format!("status {status}"),
    }
}
