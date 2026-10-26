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

//! What one receive from the personality's inbox is to the input pump: the
//! inbox empty, a frame that is not input, or a whole input frame. Not
//! input (a window manager's 28-byte focus notice, say) is passed over, and
//! the pump keeps draining what is queued behind it. Pure, so a host proof
//! holds it (capsule_linux_proofs, inbox_take_tests).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Took {
    /// Nothing waiting, or the receive failed: the pass ends.
    Done,
    /// A frame shorter than an input frame: not ours to read.
    Skip,
    /// `cap` bytes or more: an input frame, once its magic and sender agree.
    Frame,
}

/// `n`, what the receive returned; `cap`, an input frame's length.
pub fn took(n: i64, cap: usize) -> Took {
    match n {
        ..=0 => Took::Done,
        n if n < cap as i64 => Took::Skip,
        _ => Took::Frame,
    }
}
