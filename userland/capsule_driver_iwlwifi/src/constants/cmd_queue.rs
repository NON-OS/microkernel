// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The legacy host-command queue in the staging buffer.

// Host-command / transmit-queue interface. Once the firmware is alive, the
// driver hands it commands through a TFD ring per transmit queue. The
// write-pointer doorbell in the HBUS window tells the firmware a queue's new
// write index; the legacy ring holds this many descriptors (a power of two).
// Offset from iwl-prph.h / iwl-fh.h.
pub const HBUS_TARG_WRPTR: usize = 0x0460;
pub const TFD_QUEUE_SIZE: usize = 256;

// The command queue and its layout inside the DMA buffer that staged the
// firmware (reused once the firmware is alive): a TFD ring followed by a
// per-slot command area. Sized to fit within FW_STAGING_SIZE.
pub const CMD_QUEUE_ID: u8 = 4;
pub const CMD_SLOT_SIZE: usize = 512;
pub const CMD_RING_OFFSET: usize = 0;
pub const CMD_AREA_OFFSET: usize = TFD_QUEUE_SIZE * 128;
