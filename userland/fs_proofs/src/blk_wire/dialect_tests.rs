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

use super::pci::Controller;
use super::sdhci;
use super::table::{Driver, ALL};

#[test]
fn the_installer_speaks_driver_sdhci0s_wire() {
    let ops = Driver::Wire.ops();
    let sent = [ops.capacity, ops.read, ops.write, ops.flush];
    let served = [sdhci::OP_GEOMETRY, sdhci::OP_READ, sdhci::OP_WRITE, sdhci::OP_FLUSH];
    assert_eq!(sent, served);
    // The capsule's frame starts with its magic, little endian.
    let req = sdhci::Request { op: 0, flags: 0, unit: 0, request_id: 0, payload_len: 0 };
    let mut hdr = [0u8; 20];
    sdhci::encode_response_header(&mut hdr, &req, 0);
    assert_eq!(Driver::Wire.magic().to_le_bytes(), hdr[..4]);
    assert_eq!(Driver::Wire.service(0), Some(&b"driver.sdhci0"[..]));
    assert_eq!(Driver::Wire.service(1), None);
    assert_eq!(Driver::Wire.instances(), 1);
}

#[test]
fn emmc_hosts_are_the_wire_drivers_and_no_longer_sata() {
    assert!(ALL.contains(&Driver::Wire));
    assert!(Driver::Wire.serves(Controller::Emmc));
    assert!(!Driver::Ahci.serves(Controller::Emmc));
    for d in ALL {
        let emmc = d.serves(Controller::Emmc);
        assert_eq!(emmc, d == Driver::Wire, "{d:?}");
    }
    assert_eq!(Driver::Wire.label(), b"eMMC");
}
