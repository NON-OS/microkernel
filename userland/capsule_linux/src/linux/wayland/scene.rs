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
use super::reshape::Shape;
use super::scene_pixels::Pixels;
use super::state::{Buffer, Pool, Surface};
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
    pub pointer_entered: bool,
    pub keyboard_entered: bool,
    /// Whether the input router has taken this personality's subscription.
    pub subscribed: bool,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            pools: Vec::new(),
            buffers: Vec::new(),
            surfaces: Vec::new(),
            serial: 1,
            out: None,
            at: None,
            shape: None,
            window: None,
            shown: None,
            fit: Fit::default(),
            pixels: Pixels::empty(),
            pointer: None,
            keyboard: None,
            pointer_entered: false,
            keyboard_entered: false,
            subscribed: false,
        }
    }

    pub fn next_serial(&mut self) -> u32 {
        self.serial += 1;
        self.serial
    }

    /// Where the window is on screen and how large, once it is shown.
    pub fn rect(&self) -> Option<(u32, u32, u32, u32)> {
        let ((x, y), (w, h, _)) = (self.at?, self.shape?);
        Some((x, y, w, h))
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}
