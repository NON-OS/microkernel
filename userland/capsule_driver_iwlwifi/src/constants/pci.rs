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
/// The legacy staging grant. The broker maps at most 64 pages per grant for a
/// network-class device (`dma_page_limit_for_class`), so the 2 MiB this once
/// asked for was refused every time and setup never finished; the gen3 path
/// spreads its memory over several grants of at most this size instead.
pub const FW_STAGING_SIZE: u64 = 64 * 4096;
pub const PAGE_MASK: u64 = 0xFFF;
