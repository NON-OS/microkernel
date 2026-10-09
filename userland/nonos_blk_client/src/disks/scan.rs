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

//! Every disk a block driver is serving, with what is on it and what it
//! calls itself, and every controller that should be serving one and is
//! not. A driver that is registered and did not answer is listed as such
//! rather than dropped, so the list never looks complete when it is not.
//! Disks that can be installed to come first, then the rows that explain
//! what cannot.

use alloc::string::String;
use alloc::vec::Vec;

use super::booted::{Booted, By};
use super::contents::Contents;
use super::probe::unread;
use crate::device::{controllers, discover, BlockDevice, Identity};
use crate::driver::{Controller, Driver};
use crate::error::BlkError;

pub struct Disk {
    pub driver: Driver,
    /// Which of the driver's service names serves it.
    pub instance: u8,
    /// The device to install to. `None` for every row that is not a
    /// target: a driver that did not answer, a controller nothing serves,
    /// a disk that may be the one this boot came from.
    pub device: Option<BlockDevice>,
    /// Bytes, from the driver's capacity in the disk's own blocks; 0 when
    /// no driver said.
    pub size: u64,
    /// The disk's own block size in bytes; 0 when no driver said.
    pub block: u32,
    pub contents: Contents,
    pub identity: Option<Identity>,
    /// Why this row is not a target, when it is not.
    pub fault: Option<String>,
}

/// One look at the machine's disks.
pub struct Survey {
    pub disks: Vec<Disk>,
    /// An Intel RST or VMD controller is on the bus. With it on, the NVMe
    /// and SATA disks sit behind a RAID controller no NONOS driver speaks
    /// to, and the firmware's storage mode is what brings them back.
    pub raid: bool,
}

impl Survey {
    /// The rows that can be installed to.
    pub fn targets(&self) -> usize {
        self.disks.iter().filter(|d| d.device.is_some()).count()
    }

    /// Whether to tell the person to change the firmware's storage mode:
    /// only when the RAID controller is there and no disk is, since a disk
    /// found means the firmware already lets this system reach one.
    pub fn raid_hides_disks(&self) -> bool {
        self.raid && self.targets() == 0
    }

    /// Whether a later look may find more: there is nothing to install to,
    /// or a row says a driver is missing or did not answer.
    pub fn incomplete(&self) -> bool {
        self.targets() == 0 || self.disks.iter().any(Disk::missing)
    }
}

pub fn scan() -> Vec<Disk> {
    survey().disks
}

/* The disk the machine booted from, by the loader's record, is left off:
 * erasing it is never what a person choosing a disk to install onto
 * means. One found only by a copy of the running loader stays on the
 * list, never as a target, so an earlier install of this build is seen
 * and not mistaken for a missing disk. */
pub fn survey() -> Survey {
    let present = controllers();
    // The SATA driver's missing rows name eMMC when that is all it serves.
    let emmc_only = present.contains(&Controller::Emmc) && !present.contains(&Controller::Ahci);
    let booted = Booted::ask();
    let mut out = Vec::new();
    for found in discover(&present) {
        let (driver, instance) = (found.driver, found.instance);
        let row = match found.device {
            Ok(device) => match booted.by(&device) {
                Some(By::Partition) => continue,
                Some(By::Loader) => withheld(device),
                None => match Contents::probe(&device) {
                    Contents::Unread(status) => unread(device, status),
                    contents => Disk {
                        driver,
                        instance,
                        device: Some(device),
                        size: device.bytes(),
                        block: device.geometry.lba_size(),
                        contents,
                        identity: device.identity().ok().flatten(),
                        fault: None,
                    },
                },
            },
            Err(e) => Disk {
                driver,
                instance,
                device: None,
                size: 0,
                block: 0,
                contents: Contents::Unknown,
                identity: None,
                fault: Some(fault(driver, instance, &e, emmc_only)),
            },
        };
        out.push(row);
    }
    out.sort_by_key(|d| match (&d.device, d.size) {
        (Some(_), _) => 0,
        (None, s) if s > 0 => 1,
        (None, _) => 2,
    });
    let raid = present.contains(&Controller::IntelRaid);
    Survey { disks: out, raid }
}

fn withheld(device: BlockDevice) -> Disk {
    Disk {
        driver: device.driver,
        instance: device.instance,
        device: None,
        size: device.bytes(),
        block: device.geometry.lba_size(),
        contents: Contents::probe(&device),
        identity: device.identity().ok().flatten(),
        fault: Some(String::from(
            "holds the loader this boot ran, so it may be the boot disk: not offered",
        )),
    }
}

fn fault(driver: Driver, instance: u8, e: &BlkError, emmc_only: bool) -> String {
    let bus = if driver == Driver::Ahci && emmc_only {
        "eMMC"
    } else {
        core::str::from_utf8(driver.label()).unwrap_or("disk")
    };
    match e {
        BlkError::NoService if instance == 0 => {
            alloc::format!("{bus} controller present, its driver did not come up")
        }
        BlkError::NoService => {
            alloc::format!("another {bus} controller is present, no driver is serving it")
        }
        BlkError::BlockSize(n) => {
            alloc::format!("its driver serves {n}-byte blocks, which this cannot address")
        }
        _ => alloc::format!("its driver did not answer ({e:?})"),
    }
}
