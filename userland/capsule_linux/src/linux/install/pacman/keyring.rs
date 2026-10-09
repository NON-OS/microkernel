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

//! The keys pacman packages must be signed by, pinned into this capsule when
//! it is built: the file NONOS_PACMAN_KEYRING names, as `gpg --export` wrote
//! it. The capsule is inside the measured image, so the keyring is too.

use alloc::vec::Vec;

use nonos_openpgp::{keys, Key};

const RING: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pacman-keyring.gpg"));

/// None when the image was built without one; nothing then verifies.
pub fn pinned() -> Option<Vec<Key>> {
    keys(RING)
}
