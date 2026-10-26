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

//! Host proofs, group 7 of 8: test modules mounted by path
//! from proofs.rs, each at its own file.

/// L55: init waits for the ACPI capsule's first publish pass, bounded.
#[path = "acpi_first_pass_wait/mod.rs"]
pub mod acpi_first_pass_wait;
#[path = "acs_groups/mod.rs"]
pub mod acs_groups;
#[path = "boot_bus_master/mod.rs"]
pub mod boot_bus_master;
#[path = "boot_framebuffer/mod.rs"]
pub mod boot_framebuffer;
/// An empty image read from the store is refused by its own name (B-02).
#[path = "built_gate/mod.rs"]
mod built_gate;
#[path = "claim_holders/mod.rs"]
pub mod claim_holders;
#[path = "claim_holds/mod.rs"]
pub mod claim_holds;
#[path = "claim_restore/mod.rs"]
pub mod claim_restore;
#[path = "claim_scanout/mod.rs"]
pub mod claim_scanout;
#[path = "claim_stage/mod.rs"]
pub mod claim_stage;
#[path = "claim_wipe/mod.rs"]
pub mod claim_wipe;
#[path = "confine_map_fail/mod.rs"]
pub mod confine_map_fail;
#[path = "console_gate/mod.rs"]
pub mod console_gate;
#[path = "dma_direction/mod.rs"]
pub mod dma_direction;
/// H-06(a): why there is no boot framebuffer, loader to kernel to userland.
#[path = "handoff_refusal/mod.rs"]
pub mod handoff_refusal;
#[path = "input_authority_tests.rs"]
mod input_authority_tests;
#[path = "iommu_own_id/mod.rs"]
pub mod iommu_own_id;
#[path = "iommu_release/mod.rs"]
pub mod iommu_release;
/// A direct reply carries the token of the call it answers.
#[path = "ipc_reply_token/mod.rs"]
pub mod ipc_reply_token;
#[path = "irq_bind_rollback_tests.rs"]
mod irq_bind_rollback_tests;
/// Every linker symbol the kernel declares is one its image defines.
#[path = "link_symbols/mod.rs"]
pub mod link_symbols;
