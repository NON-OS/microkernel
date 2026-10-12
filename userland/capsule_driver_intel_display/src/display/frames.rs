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

//! The scanout surfaces' memory as the CPU writes it. The capsule's are
//! DMA grants mapped write-combining; the proofs' are the model's IOVA
//! pages, so a copy is checked by what the model's scanout then shows.

pub trait Frames {
    /// Copy `src` to `offset` bytes into surface `frame` (0 or 1); false,
    /// with nothing written, when it would leave the surface.
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool;
    /// Order every copy before the flip that shows it (an sfence over
    /// write-combining memory).
    fn fence(&mut self);
}
