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

//! What the installer makes of a storage controller on the bus
//! (`nonos_blk_client/src/driver/pci.rs`). A laptop whose firmware runs
//! its disks in Intel RST or VMD mode showed the installer no disk and no
//! reason; the controller that hides them is now named by its ids, and an
//! NVMe or SATA controller whose driver gave up still gets a row.

#[path = "../../nonos_blk_client/src/driver/pci.rs"]
mod pci;

use pci::{classify, Controller, INTEL_SATA_RAID, INTEL_VMD};

#[test]
fn nvme_and_ahci_controllers_are_named_by_class_and_interface() {
    assert_eq!(classify(0x144D, 0xA808, 0x01, 0x08, 0x02), Some(Controller::Nvme));
    assert_eq!(classify(0x8086, 0xA0D3, 0x01, 0x06, 0x01), Some(Controller::Ahci));
    assert_eq!(classify(0x1B21, 0x0612, 0x01, 0x06, 0x01), Some(Controller::Ahci));
    assert_eq!(classify(0x1AF4, 0x1001, 0x01, 0x00, 0x00), Some(Controller::VirtioBlk));
    assert_eq!(classify(0x1AF4, 0x1042, 0x01, 0x00, 0x00), Some(Controller::VirtioBlk));
    /* IDE and other kinds of NVM: not served. Any SATA interface is. */
    assert_eq!(classify(0x8086, 0x7010, 0x01, 0x01, 0x80), None);
    assert_eq!(classify(0x8086, 0x2828, 0x01, 0x06, 0x00), Some(Controller::Ahci));
    assert_eq!(classify(0x8086, 0x31E3, 0x01, 0x06, 0x01), Some(Controller::Ahci), "Gemini Lake");
    assert_eq!(classify(0x1022, 0x7901, 0x01, 0x06, 0x01), Some(Controller::Ahci), "AMD FCH");
    assert_eq!(classify(0x144D, 0xA808, 0x01, 0x08, 0x03), None);
    assert_eq!(classify(0x8086, 0x9A0C, 0x06, 0x04, 0x00), None, "a bridge");
    assert_eq!(classify(0x1AF4, 0x1000, 0x02, 0x00, 0x00), None, "virtio-net");
}

#[test]
fn intel_rst_and_vmd_are_named_whatever_class_they_report() {
    for id in INTEL_VMD {
        assert_eq!(classify(0x8086, id, 0x01, 0x04, 0x00), Some(Controller::IntelRaid), "{id:04x}");
        assert_eq!(classify(0x8086, id, 0x08, 0x80, 0x00), Some(Controller::IntelRaid), "{id:04x}");
    }
    for id in INTEL_SATA_RAID {
        assert_eq!(classify(0x8086, id, 0x01, 0x04, 0x00), Some(Controller::IntelRaid), "{id:04x}");
    }
    /* Any Intel RAID-class storage function is RST. */
    assert_eq!(classify(0x8086, 0x43D6, 0x01, 0x04, 0x00), Some(Controller::IntelRaid));
    /* Another vendor's RAID card is not, and neither is an Intel id elsewhere. */
    assert_eq!(classify(0x1000, 0x0097, 0x01, 0x04, 0x00), None);
    assert_eq!(classify(0x10EC, 0x9A0B, 0x01, 0x04, 0x00), None);
}

#[path = "../../nonos_blk_client/src/device/identity_parse.rs"]
#[allow(dead_code)]
mod identity_parse;

#[test]
fn the_sata_identify_reply_names_the_part_and_its_medium() {
    let mut body = vec![0u8; 76];
    body[0..8].copy_from_slice(&500_118_192u64.to_le_bytes());
    body[8..12].copy_from_slice(&512u32.to_le_bytes());
    let model = b"WDC WD5000LPCX-24C6HT0";
    body[12] = model.len() as u8;
    body[13] = 8;
    body[16..16 + model.len()].copy_from_slice(model);
    body[56..64].copy_from_slice(b"WX61AB12");
    let id = identity_parse::parse_ahci_identity(&body).unwrap();
    assert_eq!(identity_parse::trim(&id.model), "WDC WD5000LPCX-24C6HT0");
    assert_eq!(identity_parse::trim(&id.serial), "WX61AB12");
    assert!(!id.emmc);
    body[14] = 1;
    assert!(identity_parse::parse_ahci_identity(&body).unwrap().emmc);
    /* Lengths past the fields are clamped, a short body is no identity. */
    body[12] = 200;
    body[13] = 200;
    assert!(identity_parse::parse_ahci_identity(&body).is_some());
    assert!(identity_parse::parse_ahci_identity(&body[..75]).is_none());
}

#[path = "../../../src/hardware/inventory/emmc.rs"]
#[allow(dead_code)]
mod kernel_emmc;

#[test]
fn an_intel_emmc_host_is_named_and_the_lists_agree() {
    assert_eq!(classify(0x8086, 0x31CC, 0x08, 0x05, 0x01), Some(Controller::Emmc), "Gemini Lake");
    assert_eq!(classify(0x8086, 0x31CA, 0x08, 0x05, 0x01), None, "its SD card slot");
    assert_eq!(classify(0x1022, 0x31CC, 0x08, 0x05, 0x01), None);
    let mut ours = pci::INTEL_EMMC.to_vec();
    let mut kernels = kernel_emmc::INTEL_EMMC_DEVICE_IDS.to_vec();
    ours.sort_unstable();
    kernels.sort_unstable();
    assert_eq!(ours, kernels);
}

#[path = "../../nonos_blk_client/src/status_text.rs"]
mod status_text;

#[path = "../../capsule_driver_nvme/src/protocol/errno.rs"]
#[allow(dead_code)]
mod nvme_errno;

#[path = "../../capsule_driver_ahci/src/protocol/errno.rs"]
#[allow(dead_code)]
mod ahci_errno;

#[test]
fn a_refused_request_is_named_by_what_the_device_said() {
    use status_text::describe;
    assert_eq!(describe(-5), "I/O error");
    assert_eq!(describe(-110), "the device did not answer in time");
    assert_eq!(
        describe(nvme_errno::device_status(0, 0x04)),
        "NVMe status type 0 code 0x04, data transfer error"
    );
    assert_eq!(
        describe(nvme_errno::device_status(0, 0x80)),
        "NVMe status type 0 code 0x80, LBA out of range"
    );
    assert_eq!(
        describe(nvme_errno::device_status(2, 0x80)),
        "NVMe status type 2 code 0x80, write fault"
    );
    assert_eq!(describe(nvme_errno::device_status(1, 0x99)), "NVMe status type 1 code 0x99");
    assert_eq!(
        describe(ahci_errno::ata_status(0x0451)),
        "SATA status 0x51 error 0x04, command aborted"
    );
    assert_eq!(
        describe(ahci_errno::ata_status(0x1071)),
        "SATA status 0x71 error 0x10, device fault, sector not found"
    );
    assert_eq!(
        describe(-(0x100_0000 | 25 << 16 | 0x0210)),
        "eMMC host error 0x0210 on CMD25, data timeout, ADMA"
    );
    assert_eq!(describe(-(0x200_0000 | 25 << 16 | 26)), "eMMC card reported WP_VIOLATION on CMD25");
    assert_eq!(describe(-12345), "status -12345");
    /* Every value a driver can send has words, and none collides. */
    assert_eq!(nvme_errno::device_status(7, 0xff), -0x17ff);
    assert_eq!(ahci_errno::ata_status(0xffff_ffff), -0x1_ffff);
    for s in [i32::MIN, -1, 0, 1, i32::MAX] {
        let _ = describe(s);
    }
}
