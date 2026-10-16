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
mod acpi_gpio;
#[cfg(target_arch = "x86_64")]
mod acpi_i2c;
// on-screen device census: bring-up diagnostic, silenced
// mod census;
mod claim;
mod class;
mod confine;
mod device;
pub mod dma;
mod grant;
mod holder_space;
// IRQ broker is cross-arch: x86 uses IO-APIC + MSI-X; aarch64 uses
// GICv3 SPIs; riscv64 uses PLIC external sources. Backend selection
// happens inside `irq::mod.rs` via `target_arch` cfg.
pub mod irq;
pub mod mmio;
pub mod pci;
pub(crate) mod pci_index;
mod scope;
// PIO is an x86-only instruction class. Non-x86 builds skip the
// broker submodule entirely; the syscall layer fail-closes with
// ENOSYS via `syscall/microkernel/pio/unsupported.rs`.
#[cfg(target_arch = "x86_64")]
pub mod pio;
#[cfg(not(target_arch = "x86_64"))]
mod pio_absent;
mod platform;
mod power;
mod table;
mod windows;

mod api;

pub use api::*;
