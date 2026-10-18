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

//! Which logical units are served: those REPORT LUNS names that come ready
//! with 512- or 4096-byte blocks, wire unit 0 first in LUN order.

use crate::model::{Cfg, Lu, Rig};
use crate::ufs::error::UfsError;

#[test]
fn a_unit_coming_ready_is_waited_for() {
    let mut lu = Lu::new(0, 4096, 1000);
    lu.not_ready = 20;
    let rig = Rig::new(Cfg { lus: vec![lu], ..Cfg::default() });
    assert!(rig.up(false).expect("served").unit(0).is_some());
}

#[test]
fn a_unit_with_odd_blocks_is_skipped_by_name_and_the_rest_served() {
    let lus = vec![Lu::new(3, 512, 64), Lu::new(1, 520, 64), Lu::new(0, 4096, 8)];
    let rig = Rig::new(Cfg { lus, ..Cfg::default() });
    let ufs = rig.up(false).expect("two units served");
    let luns: Vec<u8> = (0..8).filter_map(|i| ufs.unit(i)).map(|u| u.lun).collect();
    assert_eq!(luns, [0, 3], "LUN order, the odd one left out");
    assert!(rig.m.said("LU 1 not served: block size 520"));
}

#[test]
fn no_unit_to_serve_is_a_failed_bring_up() {
    let rig = Rig::new(Cfg { lus: vec![Lu::new(0, 520, 8)], ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::NoDevice));
    assert!(rig.m.said("logical units failed: no device"));
}

/// Past 2^32 blocks READ CAPACITY(10) answers 0xFFFFFFFF and (16) is asked.
#[test]
fn a_unit_past_32_bit_lbas_is_sized_by_read_capacity_16() {
    let lus = vec![Lu::new(0, 4096, (1 << 32) + 5)];
    let rig = Rig::new(Cfg { lus, ..Cfg::default() });
    assert_eq!(rig.up(false).unwrap().unit(0).unwrap().blocks, (1 << 32) + 5);
}

#[test]
fn a_unit_without_a_serial_page_has_no_serial() {
    let mut lu = Lu::new(0, 4096, 8);
    lu.serial = None;
    let rig = Rig::new(Cfg { lus: vec![lu], ..Cfg::default() });
    assert_eq!(rig.up(false).unwrap().unit(0).unwrap().serial_len, 0);
}
