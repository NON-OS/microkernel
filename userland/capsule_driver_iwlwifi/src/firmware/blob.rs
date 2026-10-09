// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use super::family::Family;
use super::gen3::select::Image;

pub struct FirmwareBlob {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

const F7265: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-7265D-29.ucode");
const F8265: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-8265-36.ucode");
const F9260: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-9260-th-b0-jf-b0-46.ucode");
const AX200: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-cc-a0-77.ucode");
const AX210: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
const SO_HR: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");

pub fn blob_for_family(family: Family) -> FirmwareBlob {
    match family {
        Family::F7265 => FirmwareBlob { name: "iwlwifi-7265D-29.ucode", bytes: F7265 },
        Family::F8265 => FirmwareBlob { name: "iwlwifi-8265-36.ucode", bytes: F8265 },
        Family::F9260 => FirmwareBlob { name: "iwlwifi-9260-th-b0-jf-b0-46.ucode", bytes: F9260 },
        Family::Ax200 => FirmwareBlob { name: "iwlwifi-cc-a0-77.ucode", bytes: AX200 },
        Family::Ax210 => FirmwareBlob { name: "iwlwifi-so-a0-gf-a0-86.ucode", bytes: AX210 },
    }
}

/// The firmware for the gen3 image `gen3::select` chose by MAC and RF type.
/// The bundled firmware for a gen3 image, or `None` for one the tree does not
/// carry yet: the discrete AX210 (ty-a0-gf-a0) and Meteor Lake (ma-b0-gf-a0)
/// are selected by `select` but their linux-firmware files are not committed,
/// so those adapters are refused by name (`Refusal::ImageNotBundled`).
pub fn gen3_blob(image: Image) -> Option<FirmwareBlob> {
    match image {
        Image::SoGf => Some(FirmwareBlob { name: "iwlwifi-so-a0-gf-a0-86.ucode", bytes: AX210 }),
        Image::SoHr => Some(FirmwareBlob { name: "iwlwifi-so-a0-hr-b0-84.ucode", bytes: SO_HR }),
        Image::TyGf | Image::MaGf => None,
    }
}

/// The platform NVM (PNVM) file for a gen3 image, when the tree carries it.
/// Linux loads it from the filesystem for an Intel-SKU GF radio of the AX210
/// family (`iwl_select_pnvm_source`); an HR radio has none. None is committed
/// yet, so the firmware runs on its built-in defaults, as Linux's does when the
/// file is missing.
pub fn gen3_pnvm(image: Image) -> Option<&'static [u8]> {
    match image {
        Image::SoGf | Image::SoHr | Image::TyGf | Image::MaGf => None,
    }
}
