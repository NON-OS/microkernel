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

//! A taken-over output as the wire server drives it: the engine's BAR0,
//! the scanout surfaces' memory, the clock its waits run on, the output
//! firmware lit and the scanout flipping between the two surfaces.

use nonos_gfx_serve::Panel;

use super::frames::Frames;
use super::modeset::Modeset;
use crate::engine::access::{Clock, Mmio};
use crate::engine::gen::Gen;
use crate::engine::scanout::Scanout;
use crate::engine::takeover::Output;

/// A flip waits for one vertical blank: 100 ms covers a 24 Hz panel with
/// room to spare, and a pipe that never blanks gives up within a FLUSH's
/// budget.
pub const FLIP_US: u64 = 100_000;

pub struct Intel<M: Mmio, F: Frames, C: Clock> {
    pub m: M,
    pub frames: F,
    pub clock: C,
    pub gen: &'static Gen,
    pub out: Output,
    pub scan: Scanout,
    /// What the monitor's EDID said; empty when it said nothing.
    pub panel: Panel,
    /// What a MODE_SET needs; None leaves the output at firmware's mode.
    pub modeset: Option<Modeset>,
    /// The first IOVA of each scanout surface now shown.
    pub bases: [u64; 2],
    /// Bumped whenever STATE says something new.
    pub generation: u32,
    /// The flags STATE answered last.
    pub flags: u32,
}
