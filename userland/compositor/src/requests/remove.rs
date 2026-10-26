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

//! SCENE_REMOVE: layers taken away, every one the sender holds or one band.

use super::read::words;
use crate::protocol::SCENE_REMOVE_REQ_LEN;

/// SCENE_REMOVE's flag: only the sender's layer in band z goes.
pub const BAND_ONLY: u32 = 1;

/// SCENE_REMOVE's body: z, then flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Remove {
    pub z: u32,
    /// Only the layer in band z, not every layer the sender holds.
    pub band_only: bool,
}

pub fn scene_remove(body: &[u8]) -> Option<Remove> {
    let [z, flags] = words::<2, SCENE_REMOVE_REQ_LEN>(body)?;
    Some(Remove { z, band_only: flags & BAND_ONLY != 0 })
}
