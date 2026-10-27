// NONOS Operating System (AGPL-3.0-or-later)

//! The AX210- and BZ-family files, linked in from
//! `nonos-bootloader/firmware/intel/` (provenance in `blob`).

pub(super) const SO_GF: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");
pub(super) const SO_GF_PNVM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0.pnvm");
pub(super) const SO_HR: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-hr-b0-84.ucode");
pub(super) const TY_GF: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-ty-a0-gf-a0-86.ucode");
pub(super) const TY_GF_PNVM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-ty-a0-gf-a0.pnvm");
pub(super) const MA_GF: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-ma-b0-gf-a0-86.ucode");
pub(super) const MA_GF_PNVM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-ma-b0-gf-a0.pnvm");
pub(super) const MA_GF4: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-ma-b0-gf4-a0-86.ucode");
pub(super) const MA_GF4_PNVM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-ma-b0-gf4-a0.pnvm");
pub(super) const GL_FM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-gl-c0-fm-c0-92.ucode");
pub(super) const GL_FM_PNVM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-gl-c0-fm-c0.pnvm");
pub(super) const BZ_FM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-bz-b0-fm-c0-92.ucode");
pub(super) const BZ_FM_PNVM: &[u8] =
    include_bytes!("../../../../../nonos-bootloader/firmware/intel/iwlwifi-bz-b0-fm-c0.pnvm");
