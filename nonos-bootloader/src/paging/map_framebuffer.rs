// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::table::boot::BootServices;

use super::constants::{PTE_NX, PTE_PCD, PTE_RW};
use super::fb_window::framebuffer_tail;
use super::mapper::map_4k_run;
use super::table::PageTable;

// Identity-map the framebuffer when it lies above the low window. Newer
// Intel iGPU firmware puts the GOP framebuffer in a 64 bit BAR at 256 or
// 384 GiB (Linux logs "efifb: framebuffer at 0x4000000000"), and AMD and
// NVIDIA cards with resizable BAR at 1 TiB and beyond, all past the 64
// GiB IDENTITY_LOW_BYTES window, so the probe dots after the CR3 switch
// and the kernel's entry markers would write through an unmapped address
// and fault before any console exists. 4 KiB pages with PCD set (UC-, PAT
// index 2): the firmware's MTRRs still decide, so a write-combining MTRR
// over the aperture keeps the mapping write-combining, while a write-back
// default can never cache the device's memory. Only the frame is mapped,
// not FrameBufferSize, which some firmware reports as the whole aperture.
pub fn map_framebuffer_identity(
    bs: &BootServices,
    pml4: PageTable,
    base: u64,
    len: u64,
) -> Result<(), &'static str> {
    match framebuffer_tail(base, len) {
        Some((start, size)) => map_4k_run(bs, pml4, start, start, size, PTE_RW | PTE_PCD | PTE_NX),
        None => Ok(()),
    }
}
