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

//! The bring-up that picks the disk to serve, SATA first.

use super::hosts::Hosts;
use super::kind::Served;
use crate::emmc;
use crate::error::reason;
use crate::setup;

/// SATA first: a disk on an AHCI port wins. With none up (no controller, or
/// only empty ports, as on an eMMC laptop whose SATA controller has nothing
/// on it) the eMMC hosts are tried in discovery order, Intel eMMC first.
/// The attempt fails, and the bring-up schedule retries it, only when
/// neither gave a disk; everything an attempt took is released by then.
/// The reason given is the eMMC host's when there was one to try, since a
/// machine with eMMC rarely has a SATA disk as well.
pub fn bring_up(hosts: &Hosts) -> Result<Served, &'static str> {
    let sata = if hosts.ahci.is_empty() { None } else { Some(setup::run(&hosts.ahci)) };
    let sata_err = match sata {
        Some(Ok(driver)) => return Ok(Served::Sata(driver)),
        Some(Err(e)) => Some(e),
        None => None,
    };
    let mut emmc_err = None;
    for host in hosts.emmc.iter() {
        match emmc::open(host) {
            Ok(opened) => return Ok(Served::Emmc(opened)),
            Err(e) => {
                emmc_err.get_or_insert(e);
            }
        }
    }
    match (emmc_err, sata_err) {
        (Some(e), _) => Err(emmc::reason(e)),
        (None, Some(e)) => Err(reason(e)),
        (None, None) => Err("no controller"),
    }
}
