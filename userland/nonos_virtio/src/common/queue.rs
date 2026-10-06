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

//! Programming one virtqueue: select it, size it, place its three ring
//! parts, find its doorbell, optionally give it an MSI-X vector, enable it.
//!
//! Legacy virtio has the device dictate the ring size and find the rings
//! from one page number; modern virtio lets the driver shrink the ring and
//! name each part's address. The drivers keep their legacy ring layouts
//! unchanged and tell a modern device the size those layouts are built for.

use super::access::CommonCfg;
use super::regs::{
    QUEUE_DESC, QUEUE_DEVICE, QUEUE_DRIVER, QUEUE_ENABLE, QUEUE_MSIX_VECTOR, QUEUE_NOTIFY_OFF,
    QUEUE_SELECT, QUEUE_SIZE,
};
use super::status::fail;
use crate::error::VirtioError;
use crate::notify::notify_offset;

/// Alignment the specification requires of each split-ring part.
const DESC_ALIGN: u64 = 16;
const DRIVER_ALIGN: u64 = 2;
const DEVICE_ALIGN: u64 = 4;

/// The mapped notify region the doorbells must fall in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotifyArea {
    pub multiplier: u32,
    pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueueSpec {
    pub index: u16,
    /// The ring size the driver's layout is built for. A device that offers
    /// more is told this size.
    pub size: u16,
    /// The smallest ring the layout still works with: `size` itself when
    /// the layout fixes ring positions, lower when it adapts to the size.
    pub min_size: u16,
    /// Device addresses (the DMA grant's) of the descriptor table, the
    /// driver (available) ring and the device (used) ring.
    pub desc: u64,
    pub driver: u64,
    pub device: u64,
    /// The MSI-X table entry this queue raises, when the driver bound one.
    pub vector: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueueReady {
    /// The ring size the device was told.
    pub size: u16,
    /// The queue's doorbell, as an offset into the mapped notify region.
    pub notify_offset: usize,
    /// Whether the device read the requested vector back. False when no
    /// vector was requested; on a refusal the device answers NO_VECTOR.
    pub vector_taken: bool,
}

/// The largest ring the device takes for queue `index` (zero: no such
/// queue). For a layout whose ring positions follow from the size, so the
/// size is known before the spec's addresses are.
pub fn queue_max(c: &impl CommonCfg, index: u16) -> u16 {
    c.w16(QUEUE_SELECT, index);
    c.r16(QUEUE_SIZE)
}

/// The size to program, given the device's maximum (zero: no such queue).
pub fn choose_size(max: u16, want: u16, min: u16) -> Result<u16, VirtioError> {
    if max == 0 {
        return Err(VirtioError::QueueMissing);
    }
    let size = core::cmp::min(max, want);
    if size == 0 || size < min {
        return Err(VirtioError::QueueTooSmall);
    }
    Ok(size)
}

/// Program and enable one queue; on any refusal the device is marked FAILED.
pub fn setup_queue(
    c: &impl CommonCfg,
    spec: &QueueSpec,
    notify: NotifyArea,
) -> Result<QueueReady, VirtioError> {
    let programmed = program(c, spec, notify);
    if programmed.is_err() {
        fail(c);
    }
    programmed
}

fn program(
    c: &impl CommonCfg,
    spec: &QueueSpec,
    notify: NotifyArea,
) -> Result<QueueReady, VirtioError> {
    if !spec.desc.is_multiple_of(DESC_ALIGN)
        || !spec.driver.is_multiple_of(DRIVER_ALIGN)
        || !spec.device.is_multiple_of(DEVICE_ALIGN)
    {
        return Err(VirtioError::RingUnaligned);
    }
    c.w16(QUEUE_SELECT, spec.index);
    let size = choose_size(c.r16(QUEUE_SIZE), spec.size, spec.min_size)?;
    let notify_offset = notify_offset(c.r16(QUEUE_NOTIFY_OFF), notify.multiplier, notify.len)
        .ok_or(VirtioError::NotifyOutOfRange)?;
    c.w16(QUEUE_SIZE, size);
    c.w64(QUEUE_DESC, spec.desc);
    c.w64(QUEUE_DRIVER, spec.driver);
    c.w64(QUEUE_DEVICE, spec.device);
    let vector_taken = match spec.vector {
        Some(v) => {
            c.w16(QUEUE_MSIX_VECTOR, v);
            c.r16(QUEUE_MSIX_VECTOR) == v
        }
        None => false,
    };
    c.w16(QUEUE_ENABLE, 1);
    if c.r16(QUEUE_ENABLE) != 1 {
        return Err(VirtioError::QueueNotEnabled);
    }
    Ok(QueueReady { size, notify_offset, vector_taken })
}
