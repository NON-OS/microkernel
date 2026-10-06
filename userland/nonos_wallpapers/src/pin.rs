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

//! One wallpaper's place in the store's collection, and the hash its bytes
//! must have. The pins are generated (pins.rs) and signed into every capsule
//! that reads them, so the device is trusted for nothing but bytes.

pub struct Pin {
    pub slug: &'static [u8],
    /// Byte offset in the collection.
    pub offset: u32,
    pub len: u32,
    pub sha256: [u8; 32],
}
