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

//! What a committed buffer does to the surface it is shown on. Pure, so the
//! host proofs hold it.
//!
//! The surface was registered once, at the first buffer's width, height and
//! stride, and every later buffer was copied into it whatever its shape. A
//! smaller one was read back at the old stride, sheared, with the rest of
//! the window the last large frame's stale pixels; a larger one did not fit
//! the frame and was never shown. A buffer of any other shape now gets a
//! surface of its own shape, shown in one submit that repaints the old and
//! the new rect, and damaged whole from then on (window_damage.rs).

/// Width, height and stride.
pub type Shape = (u32, u32, u32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reshape {
    /// No surface yet: the window is opened with this one.
    Open,
    /// The surface has this shape: the pixels are copied into it.
    Same,
    /// The surface has another shape: a new one replaces it.
    Rehome,
}

/// What a `shape` buffer does, with the surface registered at `registered`.
pub fn reshape(registered: Option<Shape>, shape: Shape) -> Reshape {
    match registered {
        None => Reshape::Open,
        Some(s) if s == shape => Reshape::Same,
        Some(_) => Reshape::Rehome,
    }
}
