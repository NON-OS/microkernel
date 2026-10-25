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

//! The neutral handoff from what the riscv64 boot path read in the device
//! tree. No NONOS loader boots riscv64 yet, so nothing was verified and the
//! measurement says so.

use super::super::arch::ArchSpecificHandoff;
use super::super::console::EarlyConsole;
use super::super::cpu::CpuTopology;
use super::super::handoff::KernelHandoff;
use super::super::measurement::Measurement;
use super::super::memory::MemoryHandoff;
use super::super::timing::TimingHandoff;
use crate::arch::riscv64::boot::info::{BootInfo, MemoryType};

impl<'a> KernelHandoff<'a> {
    pub fn from_riscv64(info: &'a BootInfo) -> Self {
        let map = info.memory_map();
        let largest = map.iter().filter(|r| r.region_type == MemoryType::Available).map(|r| r.size);
        Self {
            memory: MemoryHandoff {
                map_ptr: map.as_ptr() as u64,
                map_entries: map.len() as u32,
                largest_usable_bytes: largest.max().unwrap_or(info.ram_size),
            },
            cpus: CpuTopology { boot_cpu_id: info.boot_hart, cpu_count: info.hart_count },
            console: EarlyConsole::Uart(info.uart_base),
            framebuffer: None,
            // The device tree's timebase-frequency, which the timer took.
            timing: TimingHandoff {
                fixed_freq_hz: crate::arch::riscv64::timer::frequency_hz(),
                unix_epoch_ms: 0,
            },
            measurement: Measurement { secure_boot: false, kernel_signature_verified: false },
            arch: ArchSpecificHandoff::Riscv64 { info },
        }
    }
}
