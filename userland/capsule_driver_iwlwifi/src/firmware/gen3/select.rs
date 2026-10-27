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
//! and Raptor Lake), TY (the discrete AX210) and MA (Meteor Lake, whose name
//! takes both steps, `iwl_cfg_ma`). The BZ
//! family boots the same way in the V2 CSR layout: Gl (the discrete BE200,
//! gl-c0-fm-c0) and Bz (the BE201 CNVi of Lunar and Arrow Lake, bz-b0-fm-c0,
//! its step read from the CNVi by `bz_step`), both with the FM radio.
//! Whether an image is bundled is the blob table's answer
//! (`firmware::gen3_blob`), so a known adapter whose image is not in the tree
//! is refused by name rather than started on a firmware it does not run.

use super::csr::CsrLayout;

pub mod bz_step;
mod ids;
mod qu;
mod refusal;
mod transports;

pub use bz_step::bz_mac_step;
use ids::*;
pub use ids::{mac_step, mac_type, rf_step, rf_type, MAC_BZ, MAC_GL};
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
    /// iwlwifi-ty-a0-gf-a0-86.ucode (the discrete AX210).
    TyGf,
    /// iwlwifi-ma-b0-gf-a0-86.ucode (Meteor Lake with a GF radio).
    MaGf,
    /// iwlwifi-gl-c0-fm-c0-92.ucode (BE200).
    GlFm,
    /// iwlwifi-bz-b0-fm-c0-92.ucode (BE201).
    BzFm,
    /// iwlwifi-ma-b0-gf4-a0-86.ucode (Meteor Lake with a dual-radio GF4).
    MaGf4,
    /// iwlwifi-Qu-b0-hr-b0-77.ucode, -Qu-c0-hr-b0-77, -QuZ-a0-hr-b0-77 (the
    /// AX201 CNVi of Ice, Comet, Tiger and Jasper Lake) and -cc-a0-77 (AX200).
    QuB0Hr,
    QuC0Hr,
    QuZHr,
    CcHr,
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
    if t.csr == CsrLayout::Gen2 {
        return qu::qu_image(pci_device, hw_rev, rf_id).map(|image| (image, t));
    }
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
        // iwl_cfg_ma names its image from both steps: ma-b0-gf-a0, or gf4
        // for a dual-radio module.
        MAC_MA if mac_step(hw_rev) != STEP_B => {
            return Err(Refusal::MacStepNotBundled(mac_step(hw_rev)))
        }
        MAC_MA if rf == RF_GF && rf_step(rf_id) != STEP_A => {
            return Err(Refusal::RfStepNotBundled(rf_step(rf_id)))
        }
        MAC_MA if rf == RF_GF => [Image::MaGf, Image::MaGf4][usize::from(cdb)],
        // ma-b0-hr-b0 exists upstream but is not carried.
        MAC_MA => return Err(Refusal::RfNotBundled(rf)),
        MAC_GL => fm(hw_rev, rf_id, STEP_C, Image::GlFm)?,
        MAC_BZ => fm(hw_rev, rf_id, STEP_B, Image::BzFm)?,
        _ => return Err(Refusal::MacNotBundled(mac)),
    };
    Ok((image, t))
}
