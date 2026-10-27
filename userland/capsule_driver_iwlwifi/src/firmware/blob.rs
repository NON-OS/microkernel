// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The firmware the tree carries. Every file is byte for byte linux-firmware's
//! intel/iwlwifi/ copy at bf992df7a95b5e19bb8f8546eb4d7c53ab2598d0 (WHENCE:
//! "Licence: Redistributable. See LICENCE.iwlwifi_firmware", its text in
//! `nonos-bootloader/firmware/intel/LICENSE`), pinned by sha256 in
//! scripts/baselines/upstream.sha256.

use super::gen3::select::Image;
use files::*;

// By path, so the proofs, which mount this file alone, find it too.
#[path = "blob/files.rs"]
mod files;

pub struct FirmwareBlob {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

/// One carried image: its name, its bytes, and its platform NVM file.
type Carried = (Image, &'static str, &'static [u8], Option<&'static [u8]>);

const CARRIED: &[Carried] = &[
    (Image::SoGf, "iwlwifi-so-a0-gf-a0-86.ucode", SO_GF, Some(SO_GF_PNVM)),
    (Image::SoHr, "iwlwifi-so-a0-hr-b0-84.ucode", SO_HR, None),
    (Image::TyGf, "iwlwifi-ty-a0-gf-a0-86.ucode", TY_GF, Some(TY_GF_PNVM)),
    (Image::MaGf, "iwlwifi-ma-b0-gf-a0-86.ucode", MA_GF, Some(MA_GF_PNVM)),
    (Image::MaGf4, "iwlwifi-ma-b0-gf4-a0-86.ucode", MA_GF4, Some(MA_GF4_PNVM)),
    (Image::GlFm, "iwlwifi-gl-c0-fm-c0-92.ucode", GL_FM, Some(GL_FM_PNVM)),
    (Image::BzFm, "iwlwifi-bz-b0-fm-c0-92.ucode", BZ_FM, Some(BZ_FM_PNVM)),
];

fn carried(image: Image) -> Option<&'static Carried> {
    CARRIED.iter().find(|c| c.0 == image)
}

/// The firmware for the image `gen3::select` chose, or `None` for one the
/// tree does not carry, which is then refused by name.
pub fn gen3_blob(image: Image) -> Option<FirmwareBlob> {
    carried(image).map(|c| FirmwareBlob { name: c.1, bytes: c.2 })
}

/// The platform NVM (PNVM) file for an image, when the tree carries it.
/// Linux loads one for an Intel-SKU GF radio of the AX210 family
/// (`iwl_select_pnvm_source`) and for the BZ family's FM radio; an HR radio
/// has none.
pub fn gen3_pnvm(image: Image) -> Option<&'static [u8]> {
    carried(image).and_then(|c| c.3)
}
