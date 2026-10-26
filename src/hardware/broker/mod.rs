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
// GPIO and I2C controllers only firmware knows of. ACPI describes them in AML, an x86 firmware
// interface; the same parts on an ARM board come out of the device tree, through `arch::fdt`.
crate::arch::acpi_only! {
    pub mod acpi;
    mod acpi_gpio;
    mod acpi_i2c;
}
mod api;
mod bootfb;
mod brief;
// mod census; // on-screen device census: bring-up diagnostic, silenced
pub(crate) mod claim;
mod class;
mod confine;
pub mod described;
mod device;
pub mod dma;
pub mod firmware_flash;
mod give_back;
// What the broker gives the rest of the kernel, kept beside it (and api.rs).
mod exports;
pub mod external_range;
mod grant;
mod holder;
mod holder_choice;
mod holder_space;
// IRQ broker is cross-arch: x86 uses IO-APIC + MSI-X; aarch64 uses GICv3 SPIs; riscv64 uses
// PLIC external sources. Backend selection happens inside `irq::mod.rs` via `target_arch` cfg.
pub mod irq;
mod legacy;
pub mod mmio;
pub mod pci;
pub(crate) mod pci_index;
// PIO is an x86-only instruction class. Non-x86 builds skip the broker submodule entirely; the
// syscall layer fail-closes with ENOSYS via `syscall/microkernel/pio/unsupported.rs`.
#[cfg(target_arch = "x86_64")]
pub mod pio;
#[cfg(not(target_arch = "x86_64"))]
mod pio_absent;
#[cfg(not(target_arch = "x86_64"))]
mod pio_absent_claim;
mod platform;
mod power;
mod quota;
mod quota_log;
pub mod reserved;
mod scope;
mod shown;
mod table;
mod va_space;
mod va_window;
mod vmbus;
mod windows;

pub use api::*;
pub use exports::*;
