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

//! The client's scene, and the NONOS surface it ends up on.

use alloc::vec::Vec;

use super::fit::Fit;
use super::held_keys::HeldKeys;
use super::reshape::Shape;
use super::scene_pixels::Pixels;
use super::state::{Buffer, Pool, Surface};
use super::subsurface::state::Subs;
use super::window_life::Shown;

pub struct Scene {
    pub pools: Vec<Pool>,
    pub buffers: Vec<Buffer>,
    pub surfaces: Vec<Surface>,
    /// Serial for configure events, which a client echoes back.
    pub serial: u32,
    /// The NONOS surface the client's pixels end up on.
    pub out: Option<u64>,
    /// Where the window manager placed that surface's window on screen.
    pub at: Option<(u32, u32)>,
    /// The width, height and stride that surface was registered at.
    pub shape: Option<Shape>,
    /// The window manager's id for the window, kept across new surfaces.
    pub window: Option<u32>,
    /// The guest surface, and its toplevel, the window shows.
    pub shown: Option<Shown>,
    /// Full screen and back (fit.rs).
    pub fit: Fit,
    /// This capsule's own copy of those pixels, which the surface
    /// descriptor points at.
    pub pixels: Pixels,
    pub pointer: Option<u32>,
    pub keyboard: Option<u32>,
    /// The surface the pointer and the keyboard were last told they are on
    /// (focus_step.rs).
    pub pointer_on: Option<u32>,
    pub keyboard_on: Option<u32>,
    /// Whether the input router has taken this personality's subscription.
    pub subscribed: bool,
    /// The surface told it is on the output (output_enter.rs).
    pub on_output: Option<u32>,
    /// The keys down now, so a driver's repeat is not sent as a press.
    pub held: HeldKeys,
    /// Surfaces placed on the window's (subsurface/).
    pub subs: Subs,
}
