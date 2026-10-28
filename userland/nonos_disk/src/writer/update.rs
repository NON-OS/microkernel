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

//! The blocking update, as `install` is for a whole disk: the plan laid
//! over the disk `read_installed` found, driven to its receipt. A caller
//! passes only what `Keep::Ready` held: a volume not handed over would not
//! open under the release written here, and the release it opened under
//! is the one this replaces.

use super::error::WriteError;
use super::install::drive;
use super::receipt::Receipt;
use crate::image::NonosImage;
use crate::keep::Installed;
use crate::session::{Plan, Session, ENTROPY_BYTES};
use crate::sink::BlockSink;
use crate::store::StoreImage;

/// `installed` is what `Keep::Ready` held; `entropy` seeds the boot
/// partition's volume id alone.
pub fn update<'a>(
    sink: &mut dyn BlockSink,
    installed: &Installed,
    image: &NonosImage<'a>,
    store: StoreImage,
    entropy: [u8; ENTROPY_BYTES],
    progress: &mut dyn FnMut(u64),
) -> Result<Receipt<'a>, WriteError> {
    drive(sink, Session::new(Plan::keep(installed, image, store, entropy)?), progress)
}
