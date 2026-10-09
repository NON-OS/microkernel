// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Indirect peripheral access and the CPU release.

// Peripheral (PRPH) indirect register access through the HBUS window, and the
// CPU-release register that starts the loaded firmware. After every ucode
// section is in SRAM, releasing the CPU reset boots the firmware, which then
// raises the ALIVE interrupt. Offsets from iwl-io.h / iwl-prph.h.
pub const HBUS_TARG_PRPH_WADDR: usize = 0x0444;
pub const HBUS_TARG_PRPH_WDAT: usize = 0x044C;
pub const PRPH_WADDR_ADDR_MASK: u32 = 0x000F_FFFF;
pub const PRPH_WADDR_WORD_ENABLE: u32 = 0x3 << 24;
pub const RELEASE_CPU_RESET: u32 = 0x300C;
pub const RELEASE_CPU_RESET_BIT: u32 = 0x0100_0000;
