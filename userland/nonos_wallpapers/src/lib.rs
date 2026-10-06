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

//! The wallpaper collection as the store carries it, and the pins each
//! wallpaper in it is held to. A live image's store carries the whole
//! collection as one streamed entry; an installed disk carries what was
//! kept: the collection when every wallpaper was, else each kept one alone.

#![no_std]

extern crate alloc;

mod pin;
mod pins;

use alloc::format;
use alloc::string::String;

pub use pin::Pin;
pub use pins::PINS;

/// The store entry holding every wallpaper end to end, in catalog order.
pub const COLLECTION: &str = "/Wallpapers/collection";

/// The store entry holding one kept wallpaper alone.
pub fn file_path(pin: &Pin) -> String {
    let slug = core::str::from_utf8(pin.slug).unwrap_or("");
    format!("/Wallpapers/{slug}.jpg")
}
