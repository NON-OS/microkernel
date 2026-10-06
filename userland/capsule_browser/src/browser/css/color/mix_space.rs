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

/// A color-mix() interpolation space. The xyz spaces are linear maps of
/// linear sRGB, so premultiplied mixing there equals mixing in `Linear`.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Space {
    Srgb,
    Linear,
    Lab,
    Oklab,
    Lch,
    Oklch,
    Hsl,
    Hwb,
}

impl Space {
    pub(super) fn named(s: &str) -> Option<Space> {
        Some(match s {
            "srgb" => Space::Srgb,
            "srgb-linear" | "xyz" | "xyz-d65" | "xyz-d50" => Space::Linear,
            "lab" => Space::Lab,
            "oklab" => Space::Oklab,
            "lch" => Space::Lch,
            "oklch" => Space::Oklch,
            "hsl" => Space::Hsl,
            "hwb" => Space::Hwb,
            _ => return None,
        })
    }

    /// The slot holding the hue in a polar space.
    pub(super) fn hue(self) -> Option<usize> {
        match self {
            Space::Lch | Space::Oklch => Some(2),
            Space::Hsl | Space::Hwb => Some(0),
            _ => None,
        }
    }
}
