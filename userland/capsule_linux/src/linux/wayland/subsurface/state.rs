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

//! What the server keeps of a client's subsurfaces.

use alloc::vec::Vec;

use super::blend::Plane;
use super::order::Stack;

/// A subsurface's pixels, copied out of the guest at its commit: its
/// buffer is released then, as every other surface's is.
pub struct Kept {
    pub px: Vec<u8>,
    pub plane: Plane,
}

pub struct Sub {
    /// The wl_subsurface.
    pub id: u32,
    pub surface: u32,
    pub parent: u32,
    /// Where it sits on its parent.
    pub at: (i32, i32),
    /// set_position's, taken at the parent's next commit.
    pub next_at: Option<(i32, i32)>,
    /// A synchronized subsurface's commit waits for its parent's.
    pub sync: bool,
    /// What it shows; None shows nothing, nor anything placed on it.
    pub shown: Option<Kept>,
    /// A commit waiting for the parent's: Some(None) takes it down.
    pub waiting: Option<Option<Kept>>,
}

#[derive(Default)]
pub struct Subs {
    pub list: Vec<Sub>,
    pub stacks: Vec<Stack>,
    /// place_above (true) and place_below, as (surface, reference, above),
    /// taken at the parent's next commit.
    pub restack: Vec<(u32, u32, bool)>,
    /// The window's own pixels before any subsurface was laid on them, kept
    /// while one shows, so a desynchronized commit repaints without them.
    pub base: Vec<u8>,
}

impl Subs {
    pub fn of(&self, surface: u32) -> Option<&Sub> {
        self.list.iter().find(|s| s.surface == surface)
    }

    pub fn of_mut(&mut self, surface: u32) -> Option<&mut Sub> {
        self.list.iter_mut().find(|s| s.surface == surface)
    }

    /// The bytes kept now, waiting or shown.
    pub fn kept(&self) -> usize {
        let len = |k: &Option<Kept>| k.as_ref().map_or(0, |k| k.px.len());
        self.list.iter().map(|s| len(&s.shown) + s.waiting.as_ref().map_or(0, len)).sum()
    }
}
