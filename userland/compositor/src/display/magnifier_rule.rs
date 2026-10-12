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

//! The full-screen magnifier's lens: an output shows its part of the canvas
//! `m` quarters larger than its scale would, from an origin that keeps the
//! point it follows (the pointer, or the focused window's centre) in the
//! middle, held inside the canvas at its edges as a reader expects.

/// The screen at `z` quarters of a screen pixel per canvas pixel, showing
/// the canvas from (`ox`, `oy`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Lens {
    pub z: u32,
    pub ox: u32,
    pub oy: u32,
}

/// The lens for a view of `view` canvas pixels on a `screen`, magnified `m`
/// quarters (4 none), centred on `at` in the view.
pub fn lens(at: (u32, u32), view: (u32, u32), screen: (u32, u32), m: u32) -> Lens {
    let z = (screen.0 as u64 * m as u64 / view.0.max(1) as u64).max(1);
    let seen = |s: u32| (s as u64 * 4).div_ceil(z) as u32;
    let origin =
        |at: u32, seen: u32, view: u32| at.saturating_sub(seen / 2).min(view.saturating_sub(seen));
    Lens {
        z: z as u32,
        ox: origin(at.0, seen(screen.0), view.0),
        oy: origin(at.1, seen(screen.1), view.1),
    }
}
