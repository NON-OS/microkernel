// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The MediaTek Wi-Fi parts the mt7921e capsule starts for, by vendor and
//! device as Linux v6.12's `mt7921_pci_device_table` lists them: MT7961
//! (MT7921E, AMD RZ608), MT7922 (AMD RZ616), the ITTIM MT7922 module,
//! MT7921 0608, MT7922 0616 and MT7920. kernel_proofs holds this table to the
//! capsule's own (`capsule_driver_mt7921/src/ids.rs`).
//!
//! Pure, with no imports, so host proofs can include it by path.

pub(super) const MT7921_IDS: [(u16, u16); 6] = [
    (0x14c3, 0x7961),
    (0x14c3, 0x7922),
    (0x0b48, 0x7922),
    (0x14c3, 0x0608),
    (0x14c3, 0x0616),
    (0x14c3, 0x7920),
];
