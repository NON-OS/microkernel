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

/*
 * The desktop takes its scale from the same rule as first-boot setup and the
 * installer, the brand's own file, so the desktop's type comes out the size
 * setup's did on the same screen: 1.25 from a short side of 1000, 1.5 from
 * 1440, 2 from 2160. The compositor already hands a panel of 2560 by 1440 or
 * more a canvas half its size, and this goes by that canvas, so nothing is
 * scaled twice.
 */
#[path = "../../../capsule_install/brand/src/scale_rule.rs"]
mod brand;

/// Drawing pixels per logical pixel on a canvas `width` by `height`, in quarters.
pub fn quarters_for(width: u32, height: u32) -> u32 {
    brand::quarters_for(width, height)
}
