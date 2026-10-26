// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use super::gen3::select::Image;

pub struct FirmwareBlob {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

const SO_GF: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
/// linux-firmware bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0,
/// intel/iwlwifi/iwlwifi-so-a0-gf-a0.pnvm (pinned in
/// scripts/baselines/upstream.sha256).
const SO_GF_PNVM: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0.pnvm");
const SO_HR: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");

/// The firmware for the gen3 image `gen3::select` chose by MAC and RF type.
/// The bundled firmware for a gen3 image, or `None` for one the tree does not
/// carry yet: the discrete AX210 (ty-a0-gf-a0) and Meteor Lake (ma-b0-gf-a0)
/// are selected by `select` but their linux-firmware files are not committed,
/// so those adapters are refused by name (`Refusal::ImageNotBundled`).
pub fn gen3_blob(image: Image) -> Option<FirmwareBlob> {
    match image {
        Image::SoGf => Some(FirmwareBlob { name: "iwlwifi-so-a0-gf-a0-86.ucode", bytes: SO_GF }),
        Image::SoHr => Some(FirmwareBlob { name: "iwlwifi-so-a0-hr-b0-84.ucode", bytes: SO_HR }),
        Image::TyGf | Image::MaGf => None,
    }
}

/// The platform NVM (PNVM) file for a gen3 image, when the tree carries it.
/// Linux loads it from the filesystem for an Intel-SKU GF radio of the AX210
/// family (`iwl_select_pnvm_source`); an HR radio has none. The SO GF file
/// (the AX211) is linux-firmware's; the TY and MA images are not bundled, so
/// neither is their PNVM.
pub fn gen3_pnvm(image: Image) -> Option<&'static [u8]> {
    match image {
        Image::SoGf => Some(SO_GF_PNVM),
        Image::SoHr | Image::TyGf | Image::MaGf => None,
    }
}
