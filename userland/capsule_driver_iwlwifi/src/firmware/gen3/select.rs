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

//! Which firmware an AX210-family adapter runs, and how its transport is set.
//!
//! The PCI device ID names the platform and its transport values; the firmware
//! is chosen by the MAC type and step in CSR_HW_REV and the RF module type in
//! CSR_HW_RF_ID, as Linux's `iwl_drv_get_fw_name` builds the name
//! (`<mac>-<step>0-<rf>-<step>0`). The AX210 family's three MACs share the gen3
//! boot this driver runs: SO (integrated, AX211 and AX201 on Alder and Raptor
//! Lake), TY (the discrete AX210, 0x2725) and MA (Meteor Lake, 0x7E40 and
//! 0x2729). Each image is chosen here; whether it is bundled is the blob
//! table's answer (`firmware::gen3_blob`), so a known adapter whose image is
//! not in the tree is refused by name rather than started on a firmware it
//! does not run, which would hang the chip at boot. The per-device transport
//! values (LTR delay, crystal latency, IMR) are cfg/ax210.c's
//! `iwl_so*_mac_cfg`, `iwl_ty_mac_cfg` and `iwl_ma_mac_cfg`.

/// A gen3 firmware image; the status detail carries its number.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Image {
    /// iwlwifi-so-a0-gf-a0-86.ucode (AX211), 6 GHz capable.
    SoGf,
    /// iwlwifi-so-a0-hr-b0-84.ucode (AX201 module on an SO platform).
    SoHr,
    /// iwlwifi-ty-a0-gf-a0 (the discrete AX210).
    TyGf,
    /// iwlwifi-ma-b0-gf-a0 (Meteor Lake with a GF radio).
    MaGf,
}

/// Why an adapter does not get a gen3 firmware.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// The PCI device is not one of the AX210-family platforms this driver
    /// boots (SO, TY or MA).
    NotSoDevice(u16),
    /// The MAC is not one this driver has a firmware name for.
    MacNotBundled(u16),
    /// A Meteor Lake MAC of a step other than B (Linux would load ma-a0 or
    /// ma-c0, neither of which is published at the API this driver speaks).
    MacStepNotBundled(u8),
    /// The image this adapter runs is known but not in the tree.
    ImageNotBundled(Image),
    /// The RF type has no bundled firmware on this MAC (JF, or a blank OTP).
    RfNotBundled(u16),
    /// A dual-radio (CDB) GF module needs so-a0-gf4-a0, not bundled.
    CdbNotBundled,
}

/// The transport values for one PCI device (cfg/ax210.c).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Transport {
    pub integrated: bool,
    /// SOC_CONFIGURATION LTR apply delay (1: 200 us, 2: 2500 us).
    pub ltr_delay: u8,
    pub xtal_latency: u32,
    pub low_latency_xtal: bool,
    pub imr_enabled: bool,
}

const MAC_SO: u16 = 0x37;
const MAC_SOF: u16 = 0x43;
const MAC_TY: u16 = 0x42;
const MAC_MA: u16 = 0x44;
/// `hw_rev_step` for the 8000 family on: the low nibble of CSR_HW_REV, read as
/// the step letter `'a' + n` (`iwl_drv_get_step`). Meteor Lake ships B0.
const MA_STEP_B: u8 = 1;
const RF_HR2: u16 = 0x10A;
const RF_HR1: u16 = 0x10C;
const RF_GF: u16 = 0x10D;

const SO: Transport = Transport {
    integrated: true,
    ltr_delay: 1,
    xtal_latency: 500,
    low_latency_xtal: false,
    imr_enabled: false,
};
const SO_LONG: Transport = Transport {
    integrated: true,
    ltr_delay: 2,
    xtal_latency: 12000,
    low_latency_xtal: true,
    imr_enabled: false,
};
const SO_LONG_IMR: Transport = Transport { imr_enabled: true, ..SO_LONG };
/// `iwl_ty_mac_cfg`: a discrete card, no LTR delay.
const TY: Transport = Transport {
    integrated: false,
    ltr_delay: 0,
    xtal_latency: 500,
    low_latency_xtal: false,
    imr_enabled: false,
};
/// `iwl_ma_mac_cfg`: integrated, with no LTR delay or crystal latency set.
const MA: Transport = Transport {
    integrated: true,
    ltr_delay: 0,
    xtal_latency: 0,
    low_latency_xtal: false,
    imr_enabled: false,
};

/// The transport for a PCI device ID (pcie/drv.c `iwl_hw_card_ids`, the Ty/So
/// and Ma rows), or `None` for one outside the AX210 family.
pub fn transport(pci_device: u16) -> Option<Transport> {
    match pci_device {
        0x7AF0 | 0x7F70 => Some(SO),
        0x51F0 | 0x54F0 => Some(SO_LONG),
        0x7A70 | 0x51F1 => Some(SO_LONG_IMR),
        0x2725 => Some(TY),
        0x2729 | 0x7E40 => Some(MA),
        _ => None,
    }
}

/// The MAC step: the low nibble of CSR_HW_REV (`hw_rev_step`).
pub fn mac_step(hw_rev: u32) -> u8 {
    (hw_rev & 0xF) as u8
}

/// `CSR_HW_REV_TYPE`: the MAC type.
pub fn mac_type(hw_rev: u32) -> u16 {
    ((hw_rev & 0x0000_FFF0) >> 4) as u16
}

/// `CSR_HW_RFID_TYPE`: the RF module type.
pub fn rf_type(rf_id: u32) -> u16 {
    ((rf_id & 0x00FF_F000) >> 12) as u16
}

fn rf_is_cdb(rf_id: u32) -> bool {
    rf_id & 0x1000_0000 != 0
}

/// The firmware for this adapter, or why there is none.
pub fn select(pci_device: u16, hw_rev: u32, rf_id: u32) -> Result<(Image, Transport), Refusal> {
    let t = transport(pci_device).ok_or(Refusal::NotSoDevice(pci_device))?;
    let mac = mac_type(hw_rev);
    let rf = rf_type(rf_id);
    let cdb = rf_is_cdb(rf_id);
    let image = match mac {
        MAC_SO | MAC_SOF => match rf {
            RF_GF if cdb => return Err(Refusal::CdbNotBundled),
            RF_GF => Image::SoGf,
            RF_HR1 | RF_HR2 if !cdb => Image::SoHr,
            _ => return Err(Refusal::RfNotBundled(rf)),
        },
        // ty-a0-gf-a0: Linux names the TY step 'a' whatever the register says.
        MAC_TY => match rf {
            RF_GF if cdb => return Err(Refusal::CdbNotBundled),
            RF_GF => Image::TyGf,
            _ => return Err(Refusal::RfNotBundled(rf)),
        },
        MAC_MA => {
            let step = mac_step(hw_rev);
            if step != MA_STEP_B {
                return Err(Refusal::MacStepNotBundled(step));
            }
            match rf {
                RF_GF if cdb => return Err(Refusal::CdbNotBundled),
                RF_GF => Image::MaGf,
                // ma-b0-hr-b0 exists upstream but is not carried.
                _ => return Err(Refusal::RfNotBundled(rf)),
            }
        }
        _ => return Err(Refusal::MacNotBundled(mac)),
    };
    Ok((image, t))
}
