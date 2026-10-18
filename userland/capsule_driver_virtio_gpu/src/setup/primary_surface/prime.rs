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

//! The primary resource on the device: made at the surface's size, backed
//! by its frames, filled from them, set on scanout 0 and flushed. Bring-up
//! runs it once; a recovery after a reset runs it again under the same id,
//! on the same frames, which the compositor still holds and draws into.

use super::state::Primary;
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::device::cmd;
use crate::device::cmd::transfer_to_host_2d::Rect;
use crate::device::virtqueue::ControlQueue;
use crate::state::FenceCounter;

pub fn show(q: &ControlQueue, fences: &FenceCounter, p: &Primary) -> Result<(), &'static str> {
    let (id, format) = (p.resource_id, VG_FORMAT_B8G8R8A8_UNORM);
    cmd::create_resource_2d(q, fences.issue(), id, format, p.width, p.height)?;
    cmd::attach_backing(q, fences.issue(), id, p.backing_addr, p.backing_len)?;
    let rect = Rect { x: 0, y: 0, width: p.width, height: p.height };
    cmd::transfer_to_host_2d(q, fences.issue(), id, rect, 0)?;
    cmd::set_scanout(q, fences.issue(), 0, id, rect)?;
    cmd::resource_flush(q, fences.issue(), id, rect)
}
