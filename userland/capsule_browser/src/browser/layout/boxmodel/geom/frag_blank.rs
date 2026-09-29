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

use crate::browser::css::BgLayer;

use super::content::Content;
use super::fragment::Fragment;
use super::stack_key::LEVELS;

impl Fragment {
    /// A fragment that paints nothing, for constructors to fill in.
    pub(crate) const BLANK: Fragment = Fragment {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
        bg: 0,
        border: [0; 4],
        border_color: 0,
        href: None,
        content: Content::None,
        z: [0; LEVELS],
        clip: None,
        clip_r: [0; 4],
        fixed: false,
        sticky: None,
        alpha: 255,
        bg_image: None,
        mask: false,
        fade: 0,
        fade_isect: false,
        fade_by: 0,
        tint: 0,
        bg_layer: BgLayer::INITIAL,
        shadow: None,
        radius: [0; 4],
        node: 0,
    };
}
