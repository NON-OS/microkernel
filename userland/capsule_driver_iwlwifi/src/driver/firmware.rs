// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The firmware image for the adapter's family, staged and loaded.

use crate::firmware::{
    blob_for_family, load_sections, release_cpu, stage_firmware, FirmwareBlob, FirmwareStageState,
};

use super::Driver;

impl Driver {
    pub fn firmware(&self) -> FirmwareBlob {
        blob_for_family(self.family)
    }

    pub fn stage_firmware(&mut self) -> Option<FirmwareStageState> {
        let fw = self.firmware();
        let state = stage_firmware(fw.bytes, self.dma_user_va, self.dma_len)?;
        self.firmware_stage = state;
        Some(state)
    }

    /// Transfer the staged firmware sections into device SRAM over the FH
    /// service channel, then release the CPU so the firmware boots. Requires
    /// stage_firmware to have run first. The caller then waits for ALIVE.
    pub fn load_firmware(&self) -> Result<u32, &'static str> {
        let staged = self.firmware_stage.staged_bytes;
        if staged == 0 {
            return Err("iwlwifi: firmware not staged");
        }
        let loaded = load_sections(&self.regs, self.dma_user_va, self.dma_device_addr, staged)?;
        release_cpu(&self.regs);
        Ok(loaded)
    }
}
