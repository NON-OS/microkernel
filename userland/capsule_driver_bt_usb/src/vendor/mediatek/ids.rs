/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The USB controllers that are MediaTek MT7921 or MT7922 Bluetooth, as
//! Linux v6.12 `btusb.c` marks them `BTUSB_MEDIATEK`: every MediaTek
//! (0e8d) device whose interface is E0/01/01, which is the only kind
//! `usb::choose` binds (0e8d:0608, 0e8d:0616 and the rest), and the
//! modules other makers built around the two chips, by vendor. Which chip
//! it is, and so which patch, the chip says itself (`chip.rs`); an MT7925
//! or older part under one of these ids is refused there.

pub const MEDIATEK: u16 = 0x0e8d;
const FOXCONN: u16 = 0x0489;
const LITE_ON: u16 = 0x04ca;
const AZUREWAVE: u16 = 0x13d3;
const ID_35F5: u16 = 0x35f5;

/// MT7921: e0c8 e0cd e0e0 e0f2; MT7922A: e0d8 e0d9 e0e2 e0e4 e0f1 e0f2
/// e0f5 e0f6 e102 (e0f2 is listed under both).
const FOXCONN_IDS: [u16; 12] = [
    0xe0c8, 0xe0cd, 0xe0e0, 0xe0f2, 0xe0d8, 0xe0d9, 0xe0e2, 0xe0e4, 0xe0f1, 0xe0f5, 0xe0f6, 0xe102,
];
/// MT7921: 3802; MT7922A: 3804 38e4.
const LITE_ON_IDS: [u16; 3] = [0x3802, 0x3804, 0x38e4];
/// MT7921: 3563 3564 3567 3578 3583 3606; MT7922: 3585; MT7922A: 3568
/// 3605 3607 3614 3615.
const AZUREWAVE_IDS: [u16; 12] = [
    0x3563, 0x3564, 0x3567, 0x3578, 0x3583, 0x3606, 0x3585, 0x3568, 0x3605, 0x3607, 0x3614, 0x3615,
];

/// Whether `vendor:product` is one of them.
pub fn ours(vendor: u16, product: u16) -> bool {
    match vendor {
        MEDIATEK => true,
        FOXCONN => FOXCONN_IDS.contains(&product),
        LITE_ON => LITE_ON_IDS.contains(&product),
        AZUREWAVE => AZUREWAVE_IDS.contains(&product),
        ID_35F5 => product == 0x7922,
        _ => false,
    }
}
