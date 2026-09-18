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

//! The volume this loader was itself loaded from.
//!
//! A machine with NONOS installed and the install stick still in it has two
//! volumes holding `kernel.bin`. Firmware chose one loader; that loader must
//! take its kernel from the same volume, or a newer stick can start an older
//! disk's kernel and the two disagree about the root the loader carries.
//! Own volume first, every other volume only if it has nothing to offer.

use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::Handle;

pub fn own_volume(bs: &BootServices) -> Option<Handle> {
    let image = bs.open_protocol_exclusive::<LoadedImage>(bs.image_handle()).ok()?;
    Some(image.device())
}

/// `handles` with the loader's own device moved to the front, if present.
pub fn own_first(bs: &BootServices, handles: &[Handle]) -> alloc::vec::Vec<Handle> {
    let own = own_volume(bs).filter(|own| handles.contains(own));
    let mut ordered = alloc::vec::Vec::with_capacity(handles.len());
    ordered.extend(own);
    ordered.extend(handles.iter().copied().filter(|h| Some(*h) != own));
    ordered
}
