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

// GPIO and I2C controllers that only firmware knows about. ACPI describes
// them in AML, which is an x86 firmware interface; the same parts on an ARM
// board come out of the device tree instead, through `arch::fdt`.
#[cfg(target_arch = "x86_64")]
pub mod acpi;
#[cfg(target_arch = "x86_64")]
mod acpi_gpio;
#[cfg(target_arch = "x86_64")]
mod acpi_i2c;
mod bootfb;
mod brief;
// on-screen device census: bring-up diagnostic, silenced
// mod census;
pub(crate) mod claim;
mod give_back;
mod class;
mod confine;
mod device;
pub mod dma;
// What the broker gives the rest of the kernel, kept beside it.
mod exports;
pub mod external_range;
mod grant;
mod holder;
// IRQ broker is cross-arch: x86 uses IO-APIC + MSI-X; aarch64 uses
// GICv3 SPIs; riscv64 uses PLIC external sources. Backend selection
// happens inside `irq::mod.rs` via `target_arch` cfg.
pub mod irq;
mod legacy;
pub mod mmio;
pub mod pci;
pub(crate) mod pci_index;
// PIO is an x86-only instruction class. Non-x86 builds skip the
// broker submodule entirely; the syscall layer fail-closes with
// ENOSYS via `syscall/microkernel/pio/unsupported.rs`.
#[cfg(target_arch = "x86_64")]
pub mod pio;
#[cfg(not(target_arch = "x86_64"))]
mod pio_absent;
#[cfg(not(target_arch = "x86_64"))]
mod pio_absent_claim;
mod platform;
mod power;
pub mod reserved;
mod table;
mod vmbus;
mod windows;

pub use exports::*;
#[cfg(not(target_arch = "x86_64"))]
pub use pio_absent::{
    pio_release_all_for_pid, pio_release_for_device as pio_release_for_device_unkeyed,
};
#[cfg(not(target_arch = "x86_64"))]
pub use pio_absent_claim::pio_release_for_device;
pub use platform::register_legacy as register_legacy_platform_devices;
pub use table::{contains, external_port_of, init_from_pci, list, list_by_class};
pub use vmbus::{register_vmbus, vmbus_held_by, VMBUS_DEVICE, VMBUS_VENDOR};
pub use windows::touches_device_window;
