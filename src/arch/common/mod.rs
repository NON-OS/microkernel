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

//! Facades and pieces more than one architecture backend shares: MMIO and
//! DMA ordering for all three, and the device-tree boot seed and console
//! UART for aarch64 and riscv64.

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
pub(crate) mod boot_entropy;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
pub(crate) mod boot_seed;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
pub(in crate::arch) mod console_uart;
pub(crate) mod dma_sync;
pub(crate) mod mmio_order;
pub(in crate::arch) mod own_core_kind;
