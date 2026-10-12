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

//! The scanout surfaces as the model's IOVA pages: a write lands in the
//! pages the GGTT points the plane at, so what a copy did is read back as
//! what the model's scanout shows. Fences are counted.

use super::shared::Shared;
use crate::display::Frames;

pub struct ModelFrames {
    pub model: Shared,
    /// Each surface's first IOVA; its pages follow it.
    pub iova: [u64; 2],
    pub len: usize,
    pub fences: usize,
}

impl Frames for ModelFrames {
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool {
        if frame > 1 || offset + src.len() > self.len {
            return false;
        }
        let mut m = self.model.0.borrow_mut();
        for (i, b) in src.iter().enumerate() {
            let at = offset + i;
            let page = self.iova[frame] + (at / 4096 * 4096) as u64;
            m.memory.entry(page).or_insert_with(|| vec![0; 4096])[at % 4096] = *b;
        }
        true
    }

    fn fence(&mut self) {
        self.fences += 1;
    }
}
