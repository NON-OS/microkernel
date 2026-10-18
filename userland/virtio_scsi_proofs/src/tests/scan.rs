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

//! The targets scanned: units in target then LUN order, each kind told
//! apart, the LUN field each request carries, and what is left by name.

use nonos_libc::logged;
use nonos_scsi::Kind;

use crate::model::sim::Unit;

use super::util::up;

#[test]
fn units_come_in_target_then_lun_order() {
    let mut t3 = Unit::disk(32, 512);
    t3.luns = vec![2, 0];
    let (u, m) = up(vec![
        ((3, 0), t3),
        ((3, 2), Unit::disk(48, 4096)),
        ((1, 0), Unit::cd(100)),
        ((0, 0), Unit::disk(64, 512)),
    ]);
    let at: Vec<(u16, u16, Kind)> =
        u.disks.iter().flatten().map(|d| (d.at.target, d.at.lun, d.kind)).collect();
    assert_eq!(
        at,
        vec![(0, 0, Kind::Disk), (1, 0, Kind::Cd), (3, 0, Kind::Disk), (3, 2, Kind::Disk)]
    );
    assert_eq!(u.disk(3).unwrap().block_bytes, 4096);
    // Target 3 LUN 2 on the wire: 1, the target, flat space LUN 2.
    let r = m.rest.borrow();
    let c = r.chains.iter().find(|c| c.lun[1] == 3 && c.lun[3] == 2).unwrap();
    assert_eq!(c.lun, [1, 3, 0x40, 2, 0, 0, 0, 0]);
}

#[test]
fn every_target_is_asked_and_an_empty_one_costs_one_request() {
    let (u, m) = up(vec![((7, 0), Unit::disk(64, 512))]);
    assert_eq!(u.count(), 1);
    let r = m.rest.borrow();
    let to_others = r.chains.iter().filter(|c| c.lun[1] != 7).count();
    assert_eq!(to_others, 255, "one REPORT LUNS to each of the other 255 targets");
    assert!(r.chains.iter().filter(|c| c.lun[1] != 7).all(|c| c.cdb[0] == 0xA0));
}

#[test]
fn a_lun_listed_past_max_lun_is_not_probed() {
    let mut t0 = Unit::disk(64, 512);
    t0.luns = vec![0, 5];
    let m = crate::model::device(vec![((0, 0), t0), ((0, 5), Unit::disk(8, 512))]);
    m.rest.borrow_mut().cfg.max_lun = 3;
    let u = super::util::up_on(&m);
    assert_eq!(u.count(), 1);
}

#[test]
fn a_listed_lun_with_nothing_behind_it_is_skipped() {
    let mut t0 = Unit::disk(64, 512);
    t0.luns = vec![0, 1];
    let (u, _) = up(vec![((0, 0), t0)]);
    assert_eq!(u.count(), 1, "LUN 1 answers qualifier 3");
}

#[test]
fn other_device_types_and_refusals_are_named() {
    let mut tape = Unit::disk(1, 512);
    tape.device_type = 1;
    let (u, _) = up(vec![((0, 0), tape), ((1, 0), Unit::disk(8, 520))]);
    assert_eq!(u.count(), 0);
    let log = logged();
    assert!(
        log.iter().any(|l| l == "driver.virtio_scsi0: target 0 lun 0: device type 1, not served\n"),
        "{log:?}"
    );
    assert!(log.iter().any(|l| l.contains("target 1 lun 0: not served, block size not served")));
}

#[test]
fn the_console_line_for_each_unit() {
    let (u, _) = up(vec![((0, 0), Unit::disk(2048, 512)), ((1, 0), Unit::cd(100))]);
    crate::say::unit(u.name, 0, &u.disk(0).unwrap());
    crate::say::unit(u.name, 1, &u.disk(1).unwrap());
    let log = logged();
    let n = log.len();
    assert_eq!(log[n - 2], "driver.virtio_scsi0: unit 0 (target 0 lun 0): QEMU QEMU HARDDISK, 2048 x 512 bytes (1 MiB)\n");
    assert_eq!(log[n - 1], "driver.virtio_scsi0: unit 1 (target 1 lun 0): QEMU QEMU CD-ROM, CD/DVD, 100 x 2048 bytes (0 MiB), read-only\n");
    assert!(!log.iter().any(|l| l.contains("drive-scsi0")), "no serial in the log");
}
