// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! What the driver holds for the adapter it claimed.

use crate::firmware::{Family, FirmwareStageState};
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub claim_epoch: u64,
    pub mmio_grant: u64,
    /// Bytes of the register window the broker mapped.
    pub mmio_len: u64,
    pub irq_grant: u64,
    pub dma_grant: u64,
    pub dma_user_va: u64,
    pub dma_device_addr: u64,
    pub dma_len: u64,
    pub hw_rev: u32,
    pub gp_cntrl: u32,
    pub rf_kill: bool,
    pub family: Family,
    pub regs: Regs,
    pub firmware_stage: FirmwareStageState,
    pub cmd_write_ptr: usize,
    pub rx_read_ptr: usize,
    /// The WPA2 handshake in progress, once a connection flow has started one.
    pub supplicant: Option<crate::wpa::supplicant::Supplicant>,
    /// The association in progress, once a connect request has started one.
    pub mlme: Option<crate::mlme::Mlme>,
    /// The CCMP packet-number counter for transmitted data frames; it must
    /// never repeat under one key, so it only advances.
    pub tx_pn: u64,
}
