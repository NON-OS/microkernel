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

//! One taken-over output (a lit pipe): its scanout flipping between two
//! surfaces, their memory, what its monitor's EDID said, what a mode set
//! on it needs, the surfaces' IOVAs, what STATE said of it last, and
//! whether its monitor is there.

use nonos_gfx_serve::Panel;

use super::frames::Frames;
use super::modeset::Modeset;
use super::plug::Plug;
use crate::engine::scanout::Scanout;
use crate::engine::takeover::Output;

pub struct Head<F: Frames> {
    pub out: Output,
    pub scan: Scanout,
    pub frames: F,
    /// Empty when the monitor said nothing.
    pub panel: Panel,
    /// None leaves the output at firmware's mode.
    pub modeset: Option<Modeset>,
    /// The first IOVA of each scanout surface now shown.
    pub bases: [u64; 2],
    /// Bumped whenever STATE says something new.
    pub generation: u32,
    /// The flags STATE answered last.
    pub flags: u32,
    /// The GGTT entries this output may use: the cursor at the first 64,
    /// the scanout surfaces above them.
    pub window: (u32, u32),
    /// The cursor image's graphics address once a shape was given.
    pub cursor: Option<u32>,
    pub plug: Plug,
}
