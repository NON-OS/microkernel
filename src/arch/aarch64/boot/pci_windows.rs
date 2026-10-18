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

//! The board's PCI windows, published once the address space that can reach
//! them exists.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::drivers::pci::{ecam, EcamWindow};
use crate::memory::addr::PhysAddr;

static ECAM_BASE: AtomicU64 = AtomicU64::new(0);
static ECAM_SIZE: AtomicU64 = AtomicU64::new(0);
static IO_BASE: AtomicU64 = AtomicU64::new(0);
static IO_PORT_BASE: AtomicU64 = AtomicU64::new(0);
static IO_SIZE: AtomicU64 = AtomicU64::new(0);

/// Remember the physical windows the board described.
///
/// Nothing is published to the accessors here. Config space sits at 256 GiB on
/// this board, which the boot map does not describe, and `BootInfo` lives in
/// the entry path's frame and is gone by the time `remap` runs.
pub(super) fn publish(info: &super::BootInfo) {
    ECAM_BASE.store(info.pci_ecam_base, Ordering::Relaxed);
    ECAM_SIZE.store(info.pci_ecam_size, Ordering::Relaxed);
    IO_BASE.store(info.pci_io_cpu_base, Ordering::Relaxed);
    IO_PORT_BASE.store(info.pci_io_port_base, Ordering::Relaxed);
    IO_SIZE.store(info.pci_io_size, Ordering::Release);
}

/// Map both windows and point the accessors at them.
///
/// Runs once the unified address space is up, which is the first moment the
/// MMIO mapper can place a window anywhere in the kernel half. A window is
/// published only after the hardware confirms it translates, so a board whose
/// address space does not describe it loses PCI rather than the boot.
/// One megabyte of config space describes one bus, so this is how many buses
/// the kernel can reach. The board advertises 256 of them, and mapping that is
/// 65536 pages of address space for devices that are not there. Sixteen covers
/// a host bridge and the bridges behind it, and an access past the window
/// reads as an empty slot rather than faulting.
const ECAM_BUSES: u64 = 16;
const ECAM_BUS_STRIDE: u64 = 0x10_0000;

pub(crate) fn remap() {
    let ecam_size = ECAM_SIZE.load(Ordering::Acquire).min(ECAM_BUSES * ECAM_BUS_STRIDE);
    if ecam_size >= ECAM_BUS_STRIDE {
        // Unmapped, nothing is published: the bus then reads as unreachable,
        // all ones to the scan, rather than as a fault it cannot recover from.
        if let Some(va) = map(ECAM_BASE.load(Ordering::Relaxed), ecam_size) {
            publish_ecam(va, ecam_size);
        }
    }

    let io_size = IO_SIZE.load(Ordering::Acquire);
    if io_size > 0 {
        match map(IO_BASE.load(Ordering::Relaxed), io_size) {
            Some(va) => crate::arch::port_io::set_io_window(
                va,
                IO_PORT_BASE.load(Ordering::Relaxed),
                io_size,
            ),
            None => crate::arch::port_io::set_io_window(0, 0, 0),
        }
    }
}

/// Map one window as device memory and hand back an address that resolves.
///
/// The translation is confirmed with `AT`, which puts the question to the
/// translation unit and leaves the answer in `PAR_EL1`, rather than by
/// dereferencing an address that may not be there.
fn map(phys: u64, size: u64) -> Option<u64> {
    let va = crate::memory::mmio::map_device_memory(PhysAddr::new(phys), size as usize).ok()?;
    let va = va.as_u64();
    crate::arch::aarch64::mmu::translation::translate_stage1_read(va).ok()?;
    Some(va)
}

/// The board's window starts at bus 0, so its base maps the first bus.
fn publish_ecam(va: u64, size: u64) {
    let end_bus = (size / ECAM_BUS_STRIDE - 1) as u8;
    if let Err(e) = ecam::publish(EcamWindow { segment: 0, start_bus: 0, end_bus, va }) {
        crate::log::warn!("[PCI] ECAM window {:#x} not published: {:?}", va, e);
    }
}
