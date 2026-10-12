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

//! GOP-framebuffer display path. When no display driver answers, the
//! compositor backs its primary surface with a page-aligned buffer it owns,
//! registers it, and presents through MkSurfacePresent. The kernel blits
//! that surface to the UEFI linear framebuffer the bootloader handed off,
//! so the desktop renders on any UEFI machine with no display driver.

use super::gop_backing::{display_dimensions, register_backing};
use crate::state::Context;

pub fn run_gop_once() -> Result<Context, &'static str> {
    let (width, height) = display_dimensions()?;
    let primary = register_backing(width, height)?;
    Ok(Context::new(primary, 0, 0, true))
}
