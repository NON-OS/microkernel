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

use alloc::vec::Vec;

use super::booted::{Booted, By};
use super::chosen::{ask, running_from, LOADER, RUNNING};
use super::contents::Contents;
pub use super::disk::{Disk, Survey};
use super::probe::unread;
use super::rows::{failed, target, withheld};
use crate::device::{controllers, discover};
use crate::driver::Controller;

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
    let chosen = ask();
    let mut out = Vec::new();
    for found in discover(&present) {
        let (driver, instance) = (found.driver, found.instance);
        let row = match found.device {
            Ok(device) => match booted.by(&device) {
                Some(By::Partition) => continue,
                Some(By::Loader) => withheld(device, LOADER),
                None if running_from(chosen, driver, instance) => withheld(device, RUNNING),
                None => match Contents::probe(&device) {
                    Contents::Unread(status) => unread(device, status),
                    contents => target(driver, instance, device, contents),
                },
            },
            Err(e) => failed(driver, instance, &e, emmc_only),
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
