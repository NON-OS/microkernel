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

// Invariant: the returned `KernelHandoff` borrows `info` for its arch-specific
// tail, so its lifetime is bounded by the boot info the entry path owns.

use super::super::arch::ArchSpecificHandoff;
use super::super::console::EarlyConsole;
use super::super::framebuffer::Framebuffer;
use super::super::handoff::KernelHandoff;
use super::builders;
use crate::arch::aarch64::boot::BootInfo;

impl<'a> KernelHandoff<'a> {
    pub fn from_aarch64(info: &'a BootInfo) -> Self {
        // The NONOS loader's, installed by the boot path when x1 held one.
        let loader = crate::boot::handoff::get_handoff();
        Self {
            memory: builders::memory(info),
            cpus: builders::cpus(info),
            console: EarlyConsole::Uart(info.uart.base),
            // A UEFI loader's GOP framebuffer when it handed one over; QEMU's
            // -kernel boot has none, and display comes up through virtio-gpu.
            framebuffer: loader.and_then(|h| h.framebuffer()).map(|fb| Framebuffer {
                base: fb.ptr,
                size: fb.size,
                width: fb.width,
                height: fb.height,
                stride: fb.stride,
                cursor_y: fb.cursor_y,
            }),
            timing: builders::timing(),
            measurement: builders::measurement(loader),
            arch: ArchSpecificHandoff::Aarch64 { info },
        }
    }
}
