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

//! The scanout surfaces as the model's IOVA pages: a copy is read back as
//! what the model's scanout shows. Fences are counted; a renewed pair takes
//! fresh IOVAs above every one used so far, and a pair given back is freed.

use super::shared::Shared;
use crate::display::Frames;

/// Where the model keeps the cursor image.
pub const CURSOR_IOVA: u64 = 0x7000_0000;

pub struct ModelFrames {
    pub model: Shared,
    /// Each surface's first IOVA; its pages follow it.
    pub iova: [u64; 2],
    pub len: usize,
    pub fences: usize,
    /// The surfaces a mode set replaced, until it settles.
    pub was: Option<([u64; 2], usize)>,
    /// Where the next renewed pair starts; None refuses to renew.
    pub next: Option<u64>,
}

impl Frames for ModelFrames {
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool {
        if frame > 1 || offset + src.len() > self.len {
            return false;
        }
        self.model.poke(self.iova[frame], offset, src);
        true
    }

    fn fence(&mut self) {
        self.fences += 1;
    }

    fn renew(&mut self, len: usize) -> Option<[u64; 2]> {
        let base = self.next?;
        let pair = [base, base + len as u64];
        self.next = Some(base + 2 * len as u64);
        self.was = Some((self.iova, self.len));
        pair.iter().for_each(|&a| self.model.0.borrow_mut().hold((a, a + len as u64)));
        (self.iova, self.len) = (pair, len);
        Some(pair)
    }

    fn cursor_image(&mut self, img: &[u8]) -> Option<u64> {
        self.model.poke(CURSOR_IOVA, 0, img);
        Some(CURSOR_IOVA)
    }

    fn settle(&mut self, keep: bool) {
        let Some((iova, len)) = self.was.take() else { return };
        let gone = if keep { (iova, len) } else { (self.iova, self.len) };
        if !keep {
            (self.iova, self.len) = (iova, len);
        }
        self.model.0.borrow_mut().free(gone.0.map(|a| (a, a + gone.1 as u64)));
    }
}
