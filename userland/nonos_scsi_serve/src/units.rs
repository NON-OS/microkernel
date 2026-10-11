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

//! The logical units one adapter serves, found on its bus.

use nonos_scsi::cdb::report_luns;
use nonos_scsi::errno::E_NXIO;
use nonos_scsi::{command, luns, probe, Addr, Dir, Disk, Hba, Probe};

use crate::say;

/// Units served per adapter; more are named and left.
pub const MAX_UNITS: usize = 16;
/// LUNs taken from one target's list.
pub const MAX_LUNS: usize = 8;

pub struct Units<H: Hba> {
    pub hba: H,
    pub disks: [Option<Disk>; MAX_UNITS],
    /// The endpoint the driver's replies go to.
    pub reply: u64,
    /// The driver's service name, for its console lines.
    pub name: &'static [u8],
}

impl<H: Hba> Units<H> {
    pub fn new(hba: H, reply: u64, name: &'static [u8]) -> Self {
        Self { hba, disks: [None; MAX_UNITS], reply, name }
    }

    pub fn count(&self) -> usize {
        self.disks.iter().flatten().count()
    }

    pub fn disk(&self, unit: u16) -> Option<Disk> {
        self.disks.get(unit as usize).copied().flatten()
    }

    /// Every target from 0 to `max_target`, and on each that answers the
    /// LUNs its REPORT LUNS lists up to `max_lun` (LUN 0 alone when it has
    /// no list), probed in that order; each disk or CD drive becomes the
    /// next unit.
    pub fn scan(&mut self, max_target: u16, max_lun: u16) {
        let mut n = 0;
        for target in 0..=max_target {
            let mut list = [0u16; MAX_LUNS];
            let found = self.luns(target, &mut list);
            let list = &mut list[..found];
            list.sort_unstable();
            for &lun in list.iter().filter(|l| **l <= max_lun) {
                let at = Addr { target, lun };
                match probe(&mut self.hba, at) {
                    Ok(d) if n < MAX_UNITS => {
                        self.disks[n] = Some(d);
                        n += 1;
                    }
                    Ok(_) => say::full(self.name, at),
                    Err(Probe::Absent) => {}
                    Err(Probe::Other(t)) => say::other(self.name, at, t),
                    Err(Probe::Refused(why)) => say::refused(self.name, at, why),
                }
            }
        }
    }

    /// The LUNs `target` lists, or LUN 0 alone; none when no target answers.
    fn luns(&mut self, target: u16, out: &mut [u16; MAX_LUNS]) -> usize {
        let at = Addr { target, lun: 0 };
        let c = report_luns(luns::ALLOC);
        match command(&mut self.hba, at, c.as_slice(), Dir::In, luns::ALLOC as usize) {
            Ok(n) => {
                let found = luns::parse(&self.hba.buffer()[..n], out);
                if found == 0 {
                    out[0] = 0;
                    return 1;
                }
                found
            }
            Err(E_NXIO) => 0,
            Err(_) => {
                out[0] = 0;
                1
            }
        }
    }
}
