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

use crate::browser::state::State;
use crate::browser::url::join;

/* What the last layout measured with that can change under unchanged
 * styles: which of the page's web faces are installed, and how many of
 * the images it laid out without a natural size now have one. A face or
 * an image landing changes it, so the next relayout lays out again. */
pub(super) fn measure(state: &State) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &key in &state.font_seen {
        let installed = crate::browser::fonts::with_face(key, |_| ()).is_some();
        h = (h ^ ((key as u64) << 1) ^ installed as u64).wrapping_mul(0x100_0000_01b3);
    }
    let sized = state.box_doc.as_ref().map_or(0, |d| {
        let base = state.base.as_ref();
        let key = |src: &str| base.map_or_else(|| src.into(), |b| join(b, src));
        d.unsized_imgs.iter().filter(|s| state.images.ready(&key(s)).is_some()).count()
    });
    (h ^ sized as u64).wrapping_mul(0x100_0000_01b3)
}
