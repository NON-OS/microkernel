// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::port::Port;
use super::region::{dma_flags, DmaRegion};
use crate::constants::ata::{DATA_BUF_BYTES, STRUCT_REGION_BYTES};
use crate::constants::timing::BRING_UP_COMMAND_MS;
use crate::error::AhciResult;
use crate::identity::Names;

impl Port {
    /// The port at register offset `base` of the HBA whose CAP reads `cap`,
    /// with its command list, command table, FIS area and data buffer
    /// mapped for the device, and nothing known yet of its disk.
    pub(super) fn map(device_id: u64, claim_epoch: u64, base: u32, cap: u32) -> AhciResult<Self> {
        let flags = dma_flags(cap);
        let region = |len| DmaRegion::map(device_id, claim_epoch, len, flags);
        Ok(Port {
            clb: region(STRUCT_REGION_BYTES)?,
            ctba: region(STRUCT_REGION_BYTES)?,
            _fb: region(STRUCT_REGION_BYTES)?,
            data: region(DATA_BUF_BYTES)?,
            base,
            capacity_sectors: 0,
            names: Names::empty(),
            last_tfd: 0,
            wedged: false,
            command_ms: BRING_UP_COMMAND_MS,
            reset_window: None,
        })
    }
}
