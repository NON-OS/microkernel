// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The legacy host-command queue in the staging buffer.

// The legacy ring held this many descriptors (a power of two).
pub const TFD_QUEUE_SIZE: usize = 256;

// The command area's layout inside the DMA buffer that staged the firmware,
// which the receive ring follows. Sized to fit within FW_STAGING_SIZE.
pub const CMD_SLOT_SIZE: usize = 512;
pub const CMD_AREA_OFFSET: usize = TFD_QUEUE_SIZE * 128;
