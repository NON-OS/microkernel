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

//! A row for a controller nothing serves comes from that controller's own
//! driver. So an eMMC host with no AHCI controller gives no SATA row saying
//! a controller is missing: only the wire's, named eMMC.

use super::pci::Controller;
use super::table::{Driver, ALL};
use super::unserved::unserved;

const KINDS: [Controller; 6] = [
    Controller::Nvme,
    Controller::Ahci,
    Controller::VirtioBlk,
    Controller::IntelRaid,
    Controller::Emmc,
    Controller::Ufs,
];

/// Every set of controllers a bus can show, each kind zero to two times.
fn buses() -> impl Iterator<Item = Vec<Controller>> {
    (0..3u32.pow(KINDS.len() as u32)).map(|mut n| {
        let mut bus = Vec::new();
        for kind in KINDS {
            bus.extend(core::iter::repeat_n(kind, (n % 3) as usize));
            n /= 3;
        }
        bus
    })
}

/// The missing rows `driver` adds on `bus` when it serves `served` disks,
/// counting each controller as a disk or not: the rule either way.
fn missing(driver: Driver, bus: &[Controller], served: u8, one_disk_each: bool) -> u8 {
    let on_bus = bus.iter().filter(|&&c| driver.serves(c)).count();
    unserved(one_disk_each, on_bus, served)
}

#[test]
fn a_missing_row_needs_its_own_controller_on_the_bus() {
    for bus in buses() {
        for driver in ALL {
            let mine = bus.iter().any(|&c| driver.serves(c));
            for (served, each) in [(0, false), (0, true), (1, true), (2, true)] {
                if !mine {
                    assert_eq!(missing(driver, &bus, served, each), 0, "{driver:?} on {bus:?}");
                }
            }
        }
        let emmc_only = bus.contains(&Controller::Emmc) && !bus.contains(&Controller::Ahci);
        if emmc_only {
            assert_eq!(missing(Driver::Ahci, &bus, 0, false), 0, "no SATA row on {bus:?}");
            assert_eq!(missing(Driver::Wire, &bus, 0, false), 1, "the eMMC host's row on {bus:?}");
        }
    }
}
