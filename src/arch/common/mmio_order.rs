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

//! MMIO register accesses and barriers with their ordering, one backend per
//! architecture: x86_64's fences, aarch64's DSBs, riscv64's FENCEs over
//! device and memory accesses. `memory::mmio::ordering` is their only user.

#[cfg(target_arch = "aarch64")]
#[path = "../aarch64/mmio_order/mod.rs"]
mod imp;
#[cfg(target_arch = "riscv64")]
#[path = "../riscv64/mmio_order/mod.rs"]
mod imp;
#[cfg(target_arch = "x86_64")]
#[path = "../x86_64/mmio_order/mod.rs"]
mod imp;

pub(crate) use imp::{
    fence_full, fence_reads, fence_writes, read_acquire, read_relaxed, write_relaxed, write_release,
};
