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

//! Which surface a pointer or keyboard must be on before an input event.
//! Pure, so the host proofs hold it.
//!
//! Input reaches this capsule only while its window has focus, and the
//! window shows one surface (window_life.rs). The client is told it is on
//! that surface, and told again when the window comes to show another, so
//! a client that replaces its window keeps its input. A surface the client
//! has destroyed is not left: it no longer has it to read the leave for.

/// What the client is told before an input event.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// No window is shown, so the event is not the client's.
    Drop,
    /// It is on the shown surface already.
    Stay,
    /// It leaves `leave`, when that is still there, and enters `on`.
    Move { leave: Option<u32>, on: u32 },
}

/// `entered` is the surface it was last told it is on, `shown` the shown
/// window's, and `alive` whether the client still holds a surface.
pub fn step(entered: Option<u32>, shown: Option<u32>, alive: impl Fn(u32) -> bool) -> Step {
    match (entered, shown) {
        (_, None) => Step::Drop,
        (Some(e), Some(s)) if e == s => Step::Stay,
        (e, Some(on)) => Step::Move { leave: e.filter(|&old| alive(old)), on },
    }
}
