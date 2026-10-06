// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Which Intel Wi-Fi generation a PCI device id is, grouped as Linux's
//! pcie/drv.c `iwl_hw_card_ids` groups it, and whether this driver boots it.
//! Only the AX210 family's gen3 context-info boot is written (SO, TY and MA
//! MACs, `gen3::select::transport`). Every other generation is named so the
//! boot log says which card the machine has and that its boot path is
//! missing, rather than leaving the card unmentioned.

use super::family::{family_for_device, Family};
use super::gen3::select::transport;

/// A card's generation in plain words, and the firmware Linux runs on it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Named {
    pub what: &'static str,
    pub linux_fw: &'static str,
    /// This driver has a boot path for it.
    pub boots: bool,
}

const fn n(what: &'static str, linux_fw: &'static str) -> Option<Named> {
    Some(Named { what, linux_fw, boots: false })
}

pub fn name(device: u16) -> Option<Named> {
    if transport(device).is_some() {
        let what = "AX210 family (AX210, AX211, AX411), gen3 context info";
        return Some(Named { what, linux_fw: "iwlwifi-so/ty/ma-*.ucode", boots: true });
    }
    match device {
        0x08B1 | 0x08B2 => n("7260 (7000 family)", "iwlwifi-7260-17.ucode"),
        0x08B3 | 0x08B4 => n("3160 (7000 family)", "iwlwifi-3160-17.ucode"),
        0x095A | 0x095B => n("7265 (7000 family)", "iwlwifi-7265D-29.ucode"),
        0x3165 | 0x3166 => n("3165 (7000 family)", "iwlwifi-7265D-29.ucode"),
        0x24FB => n("3168 (7000 family)", "iwlwifi-3168-29.ucode"),
        0x24F3 | 0x24F4 => n("8260 (8000 family)", "iwlwifi-8000C-36.ucode"),
        0x24F5 | 0x24F6 => n("4165 (8000 family)", "iwlwifi-8000C-36.ucode"),
        0x24FD => n("8265 or 8275 (8000 family)", "iwlwifi-8265-36.ucode"),
        0x2526 | 0x271B | 0x271C => {
            n("9260 or 9162 (9000 family)", "iwlwifi-9260-th-b0-jf-b0-46.ucode")
        }
        0x30DC | 0x31DC | 0x9DF0 | 0xA370 => {
            n("9461, 9462 or 9560 CNVi (9000 family)", "iwlwifi-9000-pu-b0-jf-b0-46.ucode")
        }
        0x2723 => n("AX200 (22000 family, discrete)", "iwlwifi-cc-a0-77.ucode"),
        // The Qu and QuZ MACs carry an HR radio (AX201) or a JF one (9461,
        // 9462, 9560); the RF id read at bring-up says which.
        0x02F0 | 0x06F0 | 0x34F0 | 0x3DF0 | 0x43F0 | 0x4DF0 | 0xA0F0 => n(
            "AX201 or 9560 CNVi (22000 family, Qu/QuZ MAC)",
            "iwlwifi-Qu/QuZ-*-hr-b0 or -jf-b0-77.ucode",
        ),
        0x272B | 0xA840 => n("BE200 or BE201 (BZ family, Wi-Fi 7)", "iwlwifi-gl/bz-*-fm-*.ucode"),
        _ if family_for_device(device) == Some(Family::Ax210) => {
            n("AX210 family id without transport values", "iwlwifi-so/ty/ma-*.ucode")
        }
        _ => None,
    }
}
