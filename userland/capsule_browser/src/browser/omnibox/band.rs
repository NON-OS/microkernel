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

/* Whether a band of the page area can be repainted by moving the scroll
 * down by the band's top: true only when every box scrolls with the page.
 * A fixed box keeps its screen row whatever the scroll and a stuck sticky
 * box is clamped to the top, so under a shifted scroll either one lands
 * in the wrong row of the band (or leaves a box that belongs there
 * undrawn). Each item is (fixed, sticky) for one box. */
pub fn band_safe<I: IntoIterator<Item = (bool, bool)>>(boxes: I) -> bool {
    boxes.into_iter().all(|(fixed, sticky)| !fixed && !sticky)
}
