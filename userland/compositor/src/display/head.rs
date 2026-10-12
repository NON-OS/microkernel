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

//! An output past 0 on a wire driver, mirroring the screen (PLAN G 10):
//! its primary, attached, and what the compositor keeps of it. Showing
//! one canvas across outputs changes the canvas under every client, which
//! waits on resizing (G 6); until then every further output mirrors.

use crate::sw_blitter::Surface;

#[derive(Clone, Copy)]
pub struct Head {
    pub output: u16,
    pub serial: u32,
    pub handle: u64,
    pub screen: Surface,
    /// A FLUSH found its primary replaced: ask SURFACE again.
    pub stale: bool,
}
