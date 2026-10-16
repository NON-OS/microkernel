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

//! Host proofs, group 1 of 4: test modules mounted by path
//! from lib.rs, each at its own file.

#[path = "addr_align/mod.rs"]
pub mod addr_align;
#[path = "aer_decode/mod.rs"]
pub mod aer_decode;
#[path = "amd_ivhd/mod.rs"]
pub mod amd_ivhd;
#[path = "amd_vi/mod.rs"]
pub mod amd_vi;
#[path = "bti_pad/mod.rs"]
pub mod bti_pad;
#[path = "clock_scale/mod.rs"]
pub mod clock_scale;
#[path = "confine_posture/mod.rs"]
pub mod confine_posture;
/// When firmware's DMA protection may be let go (K1).
#[path = "iommu_handover/mod.rs"]
pub mod iommu_handover;
/// A map's writes, flush and take-back (K1).
#[path = "iommu_place/mod.rs"]
pub mod iommu_place;
/// The domain each enumerated function starts in (K2).
#[path = "iommu_start/mod.rs"]
pub mod iommu_start;
#[path = "prove_job/mod.rs"]
pub mod prove_job;
#[path = "data_plan/mod.rs"]
pub mod data_plan;
#[path = "dma_memory_type/mod.rs"]
pub mod dma_memory_type;
#[path = "dma_placement/mod.rs"]
pub mod dma_placement;
#[path = "dma_pool_bitmap/mod.rs"]
pub mod dma_pool_bitmap;
#[path = "dma_sync_lines/mod.rs"]
pub mod dma_sync_lines;
/// DMAR reserved memory regions (K2).
#[path = "dmar_reserved/mod.rs"]
pub mod dmar_reserved;
#[path = "boot_screen/mod.rs"]
pub mod boot_screen;
#[path = "dma_surface/mod.rs"]
mod dma_surface;
#[path = "driver_scope/mod.rs"]
mod driver_scope;
#[path = "dma_wipe/mod.rs"]
pub mod dma_wipe;
#[path = "../../../src/hardware/broker/quota.rs"]
pub mod quota;
#[path = "efi_time/mod.rs"]
pub mod efi_time;
#[path = "emmc_hosts/mod.rs"]
pub mod emmc_hosts;
#[path = "fd_fork/mod.rs"]
pub mod fd_fork;
#[path = "firmware_arith/mod.rs"]
pub mod firmware_arith;
