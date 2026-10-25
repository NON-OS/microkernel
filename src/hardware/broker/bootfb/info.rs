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

//! What `MkBootFramebuffer` answers (L31): the framebuffer's geometry, for
//! the process holding its claim only, and for nobody once a GPU driver's
//! claim has taken its scanout, for the rest of the boot (L79).

use super::register::is_boot_framebuffer;
use super::rule::{encode, Geometry, INFO_LEN};
use crate::hardware::broker::claim;
use crate::kernel_core::init::framebuffer::{framebuffer_state, phys_window, surrendered};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootFbInfoError {
    NotBootFramebuffer,
    NotHolder,
}

pub fn info_for(pid: u32, device_id: u64) -> Result<[u8; INFO_LEN], BootFbInfoError> {
    if !is_boot_framebuffer(device_id) || surrendered() {
        return Err(BootFbInfoError::NotBootFramebuffer);
    }
    let held = claim::lookup(device_id).is_some_and(|c| c.pid == pid);
    let (Some(fb), Some(w)) = (framebuffer_state(), phys_window()) else {
        return Err(BootFbInfoError::NotBootFramebuffer);
    };
    if !held {
        return Err(BootFbInfoError::NotHolder);
    }
    let phys_mm = fb.physical_mm.map_or(0, |(wd, ht)| wd | (ht << 16));
    let (width, height, stride) = (fb.width, fb.height, fb.stride);
    Ok(encode(Geometry { width, height, stride, format: w.format, offset: w.offset, phys_mm }))
}
