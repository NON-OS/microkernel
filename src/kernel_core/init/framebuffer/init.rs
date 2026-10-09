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

use super::frame::frame;
use super::log::log_handoff_fb;
use super::marker::paint_mapped_marker;
use super::report::{log_mapped, log_refused};
use super::state::{KernelFramebuffer, FRAMEBUFFER};
use crate::boot::handoff::BootHandoffV1;
use crate::memory::addr::PhysAddr;

pub(crate) fn init_framebuffer(handoff: &BootHandoffV1) {
    let Some(fb) = handoff.framebuffer() else {
        log_refused(b"the loader handed over no framebuffer");
        return;
    };
    log_handoff_fb(fb.ptr, fb.stride, fb.pixel_format);
    let f = match frame(fb.ptr, fb.size, fb.width, fb.height, fb.stride) {
        Ok(f) => f,
        Err(r) => return log_refused(r.says()),
    };
    // Before the mapping, which takes write-combining from it, and before
    // start_secondary_cpus, whose APs copy the boot CPU's table.
    #[cfg(target_arch = "x86_64")]
    // SAFETY: the boot CPU, with no AP started yet.
    let wc = unsafe { crate::arch::x86_64::pat::program_boot() };
    #[cfg(not(target_arch = "x86_64"))]
    let wc = false;
    let Ok(base_va) = crate::memory::mmio::map_framebuffer(PhysAddr::new(f.base), f.map_len) else {
        return log_refused(b"the MMIO window would not take it");
    };
    let bgr = matches!(fb.pixel_format, 1 | 3);
    let kfb = *FRAMEBUFFER.call_once(|| KernelFramebuffer {
        width: fb.width,
        height: fb.height,
        stride: fb.stride,
        base_va,
        offset: f.offset,
        bgr,
        physical_mm: fb.physical_mm(),
    });
    log_mapped(kfb.width, kfb.height, kfb.stride, bgr, wc, kfb.hidpi_scale());
    paint_mapped_marker(base_va, f.offset, fb.stride, fb.width);
}
