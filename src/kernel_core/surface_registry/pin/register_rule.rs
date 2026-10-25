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

/*
 * Which pages a process may register as a surface.
 *
 * Registering took any page the caller could write. When an owner exits
 * while another process still maps one of its surfaces, the registry takes
 * the frames and frees them once the last holder lets go (`exit.rs`,
 * `orphans.rs`), so they must be frames nothing else will free. A device
 * register page or DMA buffer is the broker's, and the same exit scrubs and
 * frees it. A page attached from another owner's surface, or registered a
 * second time, is in another surface's frame list already. Each of those
 * reached the allocator twice, and the next owner of the frame shared it
 * with a device or with a window still being composited.
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Part of the range is a broker window: device registers or DMA.
    DeviceWindow,
    /// A frame backs another surface, live or orphaned, or the range maps one
    /// frame twice.
    AlreadyHeld,
}

/// `device`: the range touches a broker window. `new`: the range's frames,
/// sorted. `held`: every frame a live or orphaned surface holds.
pub fn may_register(
    device: bool,
    new: &[u64],
    held: impl IntoIterator<Item = u64>,
) -> Result<(), Refusal> {
    if device {
        return Err(Refusal::DeviceWindow);
    }
    if new.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(Refusal::AlreadyHeld);
    }
    for frame in held {
        if new.binary_search(&frame).is_ok() {
            return Err(Refusal::AlreadyHeld);
        }
    }
    Ok(())
}
