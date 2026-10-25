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

/* Painting the overlay and committing each frame as damage over it. */

use nonos_app_skeleton::clients::compositor::damage_commit;
use nonos_app_skeleton::PaintBuffer;

use super::surface::Surface;

impl Surface {
    pub(super) fn buffer(&self) -> PaintBuffer<'static> {
        let words = self.width as usize * self.height as usize;
        let pixels = unsafe { core::slice::from_raw_parts_mut(self.base as *mut u32, words) };
        PaintBuffer { pixels, stride_words: self.width, width: self.width, height: self.height }
    }

    pub(super) fn commit(&self, request_id: u32) {
        let _ = damage_commit(self.compositor, request_id, 0, 0, self.width, self.height);
    }
}
