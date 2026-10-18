// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use nonos_libc::{mk_device_claim, mk_device_release};

use super::command::command;

/// Claim the device and turn its register window on, with Bus Master off
/// until the chip and its DMA are ready.
pub fn claim(device_id: u64) -> Result<u64, &'static str> {
    let epoch = mk_device_claim(device_id);
    if epoch <= 0 {
        return Err("iwlwifi: device claim failed");
    }
    if !command(device_id, epoch as u64, false) {
        let _ = mk_device_release(device_id);
        return Err("iwlwifi: memory space enable failed");
    }
    Ok(epoch as u64)
}
