// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Family {
    F7265 = 1,
    F8265 = 2,
    F9260 = 3,
    Ax200 = 4,
    Ax210 = 5,
}

pub fn family_for_device(id: u16) -> Option<Family> {
    match id {
        // 7260, 3160, 7265, 3165 and 3168 (`generation` names each).
        0x08B1..=0x08B4 | 0x095A | 0x095B | 0x3165 | 0x3166 | 0x24FB => Some(Family::F7265),
        // 8260, 4165 and 8265; the ids between them are no adapter's.
        0x24F3..=0x24F6 | 0x24FD => Some(Family::F8265),
        0x2526 | 0x9DF0 | 0xA370 | 0x31DC | 0x30DC | 0x271B | 0x271C => Some(Family::F9260),
        0x2723 => Some(Family::Ax200),
        // The Qu and QuZ CNVi platforms (AX201 or 9560, `generation`) share
        // the AX200's entry so they are found and the radio can say it has no
        // boot path for them; 0xA0F0 is Tiger Lake's.
        0x34F0 | 0x3DF0 | 0x4DF0 | 0x02F0 | 0x06F0 | 0x43F0 | 0xA0F0 => Some(Family::Ax200),
        0x2725 | 0x2729 | 0x51F0 | 0x51F1 | 0x54F0 | 0xA74F | 0x272F => Some(Family::Ax210),
        // The other SO platforms the gen3 path selects firmware for
        // (cfg/ax210.c `iwl_so_trans_cfg` and `iwl_so_long_latency_imr_trans_cfg`).
        0x7A70 | 0x7AF0 | 0x7F70 => Some(Family::Ax210),
        // Meteor Lake (cfg/ax210.c `iwl_ma_mac_cfg`); 0x2729 is its other id.
        0x7E40 => Some(Family::Ax210),
        _ => None,
    }
}
