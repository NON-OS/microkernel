// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The PCI identity and the legacy staging grant.

pub const INTEL_VENDOR_ID: u16 = 0x8086;
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
// Runtime ucode images are hundreds of KB to ~1 MB; the old 64 KB staging
// buffer could not hold a single image, so staging truncated on real firmware.
pub const PAGE_MASK: u64 = 0xFFF;
