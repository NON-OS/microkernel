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

//! nonos_blk_client, which the installer lists and writes disks through,
//! holds driver.rtsx0 to the wire this driver serves: its magic, ops and
//! service name, an identify reply it parses as this driver lays it out,
//! no controller on the bus counted as a missing SD disk, and every status
//! this driver answers named in words, not as a number.

use crate::blk_driver::{Controller, Driver, ALL};
use crate::blk_identity::{parse_ahci_identity, Identity};
use crate::blk_status::describe;
use crate::protocol::header::MAGIC;
use crate::protocol::*;

#[test]
fn the_client_table_speaks_this_drivers_wire() {
    assert!(ALL.contains(&Driver::Rtsx));
    assert_eq!(Driver::Rtsx.magic(), MAGIC);
    let ops = Driver::Rtsx.ops();
    let named = (ops.capacity, ops.read, ops.write, ops.flush);
    assert_eq!(named, (OP_CAPACITY, OP_READ, OP_WRITE, OP_FLUSH));
    assert_eq!(Driver::Rtsx.service(0), Some(SERVICE_NAME.as_bytes()));
    assert_eq!((Driver::Rtsx.service(1), Driver::Rtsx.instances()), (None, 1));
    let kinds = [Controller::Nvme, Controller::Ahci, Controller::VirtioBlk, Controller::Emmc];
    assert!(kinds.iter().all(|&c| !Driver::Rtsx.serves(c)));
    assert!(Driver::Ahci.serves(Controller::Emmc) && Driver::Nvme.serves(Controller::Nvme));
    assert_eq!(Driver::Rtsx.label(), b"SD");
}

#[test]
fn the_client_parses_this_drivers_identify_reply() {
    let mut out = [0u8; IDENTIFY_LEN];
    identify::encode_identify(&mut out, 1 << 20, b"SC32G", b"1234ABCD");
    let id: Identity = parse_ahci_identity(&out).expect("the client parses it");
    assert_eq!((id.model_str(), id.serial_str(), id.emmc), ("SC32G", "1234ABCD", false));
    let client = include_str!("../../../nonos_blk_client/src/device/identity.rs");
    assert!(client.contains(&format!("const OP_SD_IDENTIFY: u16 = {OP_IDENTIFY};")));
    let client = include_str!("../../../nonos_blk_client/src/device/handle.rs");
    assert!(client.contains(&format!("const E_NOMEDIUM: i32 = {E_NOMEDIUM};")));
}

#[test]
fn every_status_this_driver_answers_is_named_in_words() {
    let all = [E_IO, E_NXIO, E_ACCES, E_NODEV, E_INVAL, E_ROFS, E_MSGSIZE, E_TIMEDOUT, E_NOMEDIUM];
    for status in all {
        assert!(!describe(status).starts_with("status "), "{status}: {}", describe(status));
    }
    assert_eq!(describe(E_ROFS), "the disk is write-protected");
    assert_eq!(describe(E_NOMEDIUM), "no medium in the drive");
}
