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

//! What the broker gives the rest of the kernel: the calls the syscall layer,
//! the exit path and the boot make, under the names they know them by
//! (x86_64's own, in one use, are in api.rs).

pub use super::bootfb::{
    info_for as boot_framebuffer_info, register_boot_framebuffer, BootFbInfoError,
};
// pub use super::census::render_and_hold as device_census; // bring-up diagnostic, silenced
pub use super::claim::{
    claim as claim_device, exit_begin as claims_exit_begin, exit_end as claims_exit_end,
    quiesce_all as quiesce_all_devices, quiesce_held as quiesce_held_device,
    release as release_device, Claim, ClaimError,
};
pub use super::class::{classify_pci, Class};
pub use super::confine::{set_acs_groups, Port as AcsPort};
pub use super::device::{
    Bar, BarKind, BusKind, DeviceRecord, BAR_FLAG_FIXED, DEVICE_FLAG_CLAIMED,
    DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_DISABLED, DEVICE_FLAG_DMA_NONCOHERENT,
    DEVICE_FLAG_EXTERNAL, DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE, NUMA_UNKNOWN,
};
pub use super::dma::{
    covers as dma_covers, free_held_run as dma_free_held_run,
    grant_run_covering as dma_grant_run_covering, map_for_caller as dma_map_for_caller,
    release_all_for_pid as dma_release_all_for_pid, release_for_device as dma_release_for_device,
    unmap_grant as dma_unmap_grant, wipe_live_grants as dma_wipe_live_grants, DmaError, DmaGrant,
    DmaMapError, DmaMapRequest, DmaMapResult,
};
pub use super::external_range::ExternalRange;
pub use super::give_back::give_back;
pub use super::grant::{GrantError, MmioGrant};
pub use super::holder::holder_of;
pub(crate) use super::holder::others_alive;
pub use super::irq::{
    ack_grant as irq_ack_grant, bind as irq_bind, poll as irq_poll,
    release_all_for_pid as irq_release_all_for_pid, release_for_device as irq_release_for_device,
    unmap_grant as irq_unmap_grant, wait_arm as irq_wait_arm, wait_disarm as irq_wait_disarm,
    IrqBindError, IrqBindRequest, IrqBindResult, IrqError, IrqPollResult,
};
pub use super::mmio::{
    map_for_caller, release_all_for_pid, release_for_device, unmap_grant, MmioMapError,
    MmioMapRequest, MmioMapResult,
};
pub use super::pci::{
    read as pci_config_read, write as pci_config_write, PciReadError, PciReadRequest,
    PciWriteError, PciWriteRequest,
};
