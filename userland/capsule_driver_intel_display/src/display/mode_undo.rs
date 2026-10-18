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

//! A mode set whose lighting failed, undone: the old mode lit again on the
//! old surfaces before the new ones are given back, since the plane may be
//! on the new ones; with no way back, the plane's GGTT entries forgotten
//! first, so it reaches no frame given back to anyone, and the output
//! marked dark until a mode set lights it.

use super::frames::Frames;
use super::head::Head;
use super::mode_light::light;
use super::modeset::Modeset;
use super::plug::Plug;
use crate::engine::access::{Clock, Mmio};

pub fn undo<F: Frames>(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    hd: &mut Head<F>,
    ms: &Modeset,
    (size, bases): ((u32, u32), [u64; 2]),
    held: bool,
) {
    match light(m, clock, &mut hd.scan, (ms, &ms.now), size, bases, held) {
        Ok(()) => hd.plug = Plug::In,
        Err(_) => {
            hd.scan.forget(m, hd.window.1);
            // Dark: STATE says LOST, so the wire server puts the mode back.
            hd.plug = Plug::Back;
        }
    }
    hd.frames.settle(false);
}
