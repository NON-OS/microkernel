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

//! The eMMC disk is served through the AHCI capsule's wire protocol. These
//! hold the parts of that contract the eMMC tree decides against the AHCI
//! capsule's own source: a request moves the same sectors.

use crate::emmc::disk::{DATA_BUF_BYTES, MAX_SECTORS, SECTOR_SIZE};
use crate::emmc::info::{names, MEDIUM_EMMC};
use crate::emmc::mmc::Cid;
use crate::{ahci_ata, ahci_identify, ahci_ops};

#[test]
fn a_request_moves_what_an_ahci_request_moves() {
    assert_eq!(MAX_SECTORS, ahci_ata::MAX_SECTORS);
    assert_eq!(SECTOR_SIZE, ahci_ata::SECTOR_SIZE);
    assert_eq!(DATA_BUF_BYTES as u64, ahci_ata::DATA_BUF_BYTES);
}

#[test]
fn an_emmc_part_is_named_through_ahcis_identify() {
    assert_eq!(ahci_ops::OP_IDENTIFY, 8);
    let cid =
        Cid { mid: 0x90, cbx: 1, oid: 0, pnm: *b"HAG4a2", prv: 0xa2, psn: 0x0102_0304, mdt: 0 };
    let n = names(&cid);
    let mut out = [0u8; ahci_identify::IDENTIFY_PAYLOAD_LEN];
    ahci_identify::encode_identify(&mut out, 122_142_720, 512, n.model(), &n.serial, MEDIUM_EMMC);
    assert_eq!(u64::from_le_bytes(out[0..8].try_into().unwrap()), 122_142_720);
    assert_eq!(out[12] as usize, b"SK hynix HAG4a2".len());
    assert_eq!(&out[16..16 + out[12] as usize], b"SK hynix HAG4a2");
    assert_eq!(&out[56..64], b"01020304");
    assert_eq!(out[14], 1, "the installer reads byte 14 as the medium");
}
