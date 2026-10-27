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

//! Which firmware an AX210- or BZ-family adapter runs, and how its transport
//! is set.
//!
//! The PCI device ID names the platform and its transport values
//! (`transports`); the firmware is chosen by the MAC type and step in
//! CSR_HW_REV and the RF type and step in CSR_HW_RF_ID, as Linux's
//! `iwl_drv_get_fwname_pre` builds the name (`<mac>-<step>0-<rf>-<step>0`).
//! The AX210 family's MACs share one gen3 boot: SO (AX211 and AX201 on Alder
//! and Raptor Lake), TY (the discrete AX210) and MA (Meteor Lake). The BZ
//! family boots the same way in the V2 CSR layout: Gl (the discrete BE200,
//! gl-c0-fm-c0) and Bz (the BE201 CNVi of Lunar and Arrow Lake, bz-b0-fm-c0,
//! its step read from the CNVi by `bz_step`), both with the FM radio.
//! Whether an image is bundled is the blob table's answer
//! (`firmware::gen3_blob`), so a known adapter whose image is not in the tree
//! is refused by name rather than started on a firmware it does not run.

use super::csr::CsrLayout;

pub mod bz_step;
mod refusal;
mod transports;

pub use bz_step::bz_mac_step;
pub use refusal::Refusal;
pub use transports::transport;

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
    /// iwlwifi-gl-c0-fm-c0-92.ucode (BE200).
    GlFm,
    /// iwlwifi-bz-b0-fm-c0-92.ucode (BE201).
    BzFm,
}

/// The transport values for one PCI device.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Transport {
    pub integrated: bool,
    /// SOC_CONFIGURATION LTR apply delay (1: 200 us, 2: 2500 us).
    pub ltr_delay: u8,
    pub xtal_latency: u32,
    pub low_latency_xtal: bool,
    pub imr_enabled: bool,
    /// The family's CSR layout (V2 for BZ).
    pub csr: CsrLayout,
}

const MAC_SO: u16 = 0x37;
const MAC_SOF: u16 = 0x43;
const MAC_TY: u16 = 0x42;
const MAC_MA: u16 = 0x44;
/// `IWL_CFG_MAC_TYPE_BZ` and `IWL_CFG_MAC_TYPE_GL`.
pub const MAC_BZ: u16 = 0x46;
pub const MAC_GL: u16 = 0x47;
/// `hw_rev_step` for the 8000 family on: the low nibble of CSR_HW_REV (for
/// Bz, the CNVi's step ORed in), read as the step letter `'a' + n`
/// (`iwl_drv_get_step`). Meteor Lake and Bz ship B0, Gl C0.
const STEP_B: u8 = 1;
const STEP_C: u8 = 2;
const RF_HR2: u16 = 0x10A;
const RF_HR1: u16 = 0x10C;
const RF_GF: u16 = 0x10D;
/// `IWL_CFG_RF_TYPE_FM`; linux-firmware's gl and bz images are for its C step.
const RF_FM: u16 = 0x112;

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

/// `CSR_HW_RFID_STEP`: the RF module step.
pub fn rf_step(rf_id: u32) -> u8 {
    ((rf_id >> 8) & 0xF) as u8
}

fn rf_is_cdb(rf_id: u32) -> bool {
    rf_id & 0x1000_0000 != 0
}

// The image an FM radio runs on a BZ-family MAC that must be of step `want`.
fn fm(hw_rev: u32, rf_id: u32, want: u8, image: Image) -> Result<Image, Refusal> {
    let (step, rf) = (mac_step(hw_rev), rf_type(rf_id));
    match rf {
        _ if step != want => Err(Refusal::MacStepNotBundled(step)),
        RF_FM if rf_is_cdb(rf_id) => Err(Refusal::CdbNotBundled),
        RF_FM if rf_step(rf_id) != STEP_C => Err(Refusal::RfStepNotBundled(rf_step(rf_id))),
        RF_FM => Ok(image),
        _ => Err(Refusal::RfNotBundled(rf)),
    }
}

/// The firmware for this adapter, or why there is none. For Bz, `hw_rev`
/// carries the CNVi's step (`bz_mac_step`), as Linux's does. A MAC of the
/// other family than its PCI device's is refused.
pub fn select(pci_device: u16, hw_rev: u32, rf_id: u32) -> Result<(Image, Transport), Refusal> {
    let t = transport(pci_device).ok_or(Refusal::NotSoDevice(pci_device))?;
    let mac = mac_type(hw_rev);
    let rf = rf_type(rf_id);
    let cdb = rf_is_cdb(rf_id);
    if matches!(mac, MAC_BZ | MAC_GL) != (t.csr == CsrLayout::V2) {
        return Err(Refusal::MacNotBundled(mac));
    }
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
            if step != STEP_B {
                return Err(Refusal::MacStepNotBundled(step));
            }
            match rf {
                RF_GF if cdb => return Err(Refusal::CdbNotBundled),
                RF_GF => Image::MaGf,
                // ma-b0-hr-b0 exists upstream but is not carried.
                _ => return Err(Refusal::RfNotBundled(rf)),
            }
        }
        MAC_GL => fm(hw_rev, rf_id, STEP_C, Image::GlFm)?,
        MAC_BZ => fm(hw_rev, rf_id, STEP_B, Image::BzFm)?,
        _ => return Err(Refusal::MacNotBundled(mac)),
    };
    Ok((image, t))
}
