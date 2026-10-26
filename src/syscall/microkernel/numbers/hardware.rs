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

//! Capabilities, devices, MMIO, IRQs, DMA, port I/O, the hypervisor and PCI.

use crate::syscall::abi::tag4;

pub const SYS_CAP_GRANT: u64 = tag4(b"MCGT");
pub const SYS_CAP_REVOKE: u64 = tag4(b"MCRV");
pub const SYS_CAP_CHECK: u64 = tag4(b"MCCK");
pub const SYS_DEVICE_LIST: u64 = tag4(b"MDLS");
pub const SYS_DEVICE_CLAIM: u64 = tag4(b"MDCL");
pub const SYS_DEVICE_RELEASE: u64 = tag4(b"MDRL");
pub const SYS_BOOT_FRAMEBUFFER: u64 = tag4(b"MBFB");
pub const SYS_MMIO_MAP: u64 = tag4(b"MMMP");
pub const SYS_MMIO_UNMAP: u64 = tag4(b"MMUM");
pub const SYS_IRQ_BIND: u64 = tag4(b"MIRB");
pub const SYS_IRQ_UNBIND: u64 = tag4(b"MIRU");
pub const SYS_IRQ_ACK: u64 = tag4(b"MIRA");
pub const SYS_IRQ_POLL: u64 = tag4(b"MIRP");
pub const SYS_IRQ_WAIT: u64 = tag4(b"MIRW");
pub const SYS_DMA_MAP: u64 = tag4(b"MDMM");
pub const SYS_DMA_UNMAP: u64 = tag4(b"MDMU");
pub const SYS_PIO_GRANT: u64 = tag4(b"MPGT");
pub const SYS_PIO_READ: u64 = tag4(b"MPRD");
pub const SYS_PIO_WRITE: u64 = tag4(b"MPWR");
pub const SYS_PIO_RELEASE: u64 = tag4(b"MPRL");
/// One allowed VMware backdoor call, on a host that answers it; needs Pio.
pub const SYS_HV_BACKDOOR: u64 = tag4(b"MHVB");
/// The VMBus core's SynIC calls; each needs Driver and the VMBus root's claim.
pub const SYS_HV_POST: u64 = tag4(b"MHVP");
pub const SYS_HV_RECV: u64 = tag4(b"MHVR");
pub const SYS_HV_EVENTS: u64 = tag4(b"MHVE");
pub const SYS_HV_SIGNAL: u64 = tag4(b"MHVS");
pub const SYS_HV_CHANNEL: u64 = tag4(b"MHVC");
pub const SYS_MK_DEBUG: u64 = tag4(b"MDBG");
pub const SYS_PCI_CONFIG_READ: u64 = tag4(b"MPCR");
pub const SYS_PCI_CONFIG_WRITE: u64 = tag4(b"MPCW");
