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

//! `crate::linux::file`, as far as the user-files path needs it.

#[path = "../../../../capsule_linux/src/linux/file/flags.rs"]
pub mod flags;
#[path = "../../../../capsule_linux/src/linux/file/resolve.rs"]
pub mod resolve;
pub mod user;

/// The family's home and its seed.
pub mod private;

/// resolve.rs maps a path to a store key; only its normaliser is used here.
pub mod root {
    pub struct Key;
    impl Key {
        pub fn under_root(_visible: &[u8]) -> Key {
            Key
        }
        pub fn in_tree(_tree: &[u8], _visible: &[u8]) -> Key {
            Key
        }
    }
}

/// resolve.rs notes a path whose `..` climbed past `/`; nothing to note here.
pub mod clamp {
    pub fn note(_path: &[u8]) {}
}
