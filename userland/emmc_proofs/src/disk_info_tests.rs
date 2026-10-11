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

//! What a disk brought up on the model says about itself.

use crate::model::{CardCfg, HostCfg, Rig};

#[test]
fn info_payloads_describe_the_card() {
    let rig = Rig::new(HostCfg::default(), CardCfg::default());
    let mut disk = rig.disk(true).expect("bring-up");
    assert_eq!(disk.capacity_sectors(), 61_071_360);
    let n = disk.names();
    assert_eq!(n.model(), b"SanDisk DF4032");
    assert_eq!(&n.serial, b"12345678");
    let c = disk.controller_info();
    assert_eq!(u32::from_le_bytes(c[0..4].try_into().unwrap()), crate::model::GLK_CAPS);
    assert_eq!(c[20], 1);
    let p = disk.port_entry();
    assert_eq!(&p[0..4], &[0, 1, 1, 5]);
    // CMD: the card's status, in transfer state and ready for data.
    assert_eq!(u32::from_le_bytes(p[16..20].try_into().unwrap()), 4 << 9 | 1 << 8);
}
