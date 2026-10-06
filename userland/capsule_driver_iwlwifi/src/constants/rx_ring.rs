// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The legacy receive ring and the end of the staging layout.

use super::cmd_queue::{CMD_AREA_OFFSET, CMD_SLOT_SIZE, TFD_QUEUE_SIZE};

// The receive ring, laid out after the command area in the same DMA buffer.
// The firmware posts packets into RX_QUEUE_SIZE receive buffers of RB_SIZE each
// and advances the write-pointer register as it does. RX_RB_OFFSET + the ring
// fits within FW_STAGING_SIZE. Register offset from iwl-fh.h.
pub const RX_QUEUE_SIZE: usize = 256;
pub const RB_SIZE: usize = 4096;
pub const RX_RB_OFFSET: usize = CMD_AREA_OFFSET + TFD_QUEUE_SIZE * CMD_SLOT_SIZE;
pub const RX_WPTR_REG: usize = 0x1BC0;

// The last byte the command ring and the receive ring occupy in the DMA buffer.
// The driver refuses to touch the DMA area unless the grant is at least this
// large, so a short grant cannot turn into an out-of-bounds access.
pub const DMA_LAYOUT_END: usize = RX_RB_OFFSET + RX_QUEUE_SIZE * RB_SIZE;
