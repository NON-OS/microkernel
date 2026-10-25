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

//! The things the included driver files take from `nonos_libc`, each
//! recorded per thread; every test runs on its own thread.

mod broker;
mod devices;
mod dma;
mod hv;
mod ipc;
mod script;
#[allow(dead_code)]
#[path = "../../../../src/arch/x86_64/hypervisor/hv/channels.rs"]
mod channels;
#[allow(dead_code)]
#[path = "../../../../src/arch/x86_64/hypervisor/hv/regs.rs"]
mod regs;
#[allow(dead_code)]
#[path = "../../../../src/arch/x86_64/hypervisor/hv/vmbus_out.rs"]
mod vmbus_out;

pub use broker::{mk_device_claim, mk_device_release};
pub use devices::{mk_device_list, set_devices, DeviceRecord, BUS_KIND_PCI, BUS_KIND_VIRT};
pub use dma::{mk_dma_map, mk_dma_unmap, script_dma, DmaMapOut, MK_DMA_MAP_COHERENT};
pub use hv::{
    mk_hv_post, mk_hv_recv, set_host, Host, MK_HV_MESSAGE_BYTES, MK_VMBUS_DEVICE, MK_VMBUS_VENDOR,
};
pub use ipc::{logged, mk_debug, mk_idle_ms, mk_yield, now_ms, Deadline};
pub use script::{calls, script, Script};
