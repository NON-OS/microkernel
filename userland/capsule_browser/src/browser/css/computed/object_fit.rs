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

use crate::browser::layout::boxmodel::Rel;

/// object-fit with its object-position: Contain letterboxes the whole
/// image and Cover crops it, each placed at the position (px plus
/// per-mille of the room left over, centred by default); Fill stretches
/// it over the box and keeps the position only for a later fit.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjectFit {
    Contain([Rel; 2]),
    Cover([Rel; 2]),
    Fill([Rel; 2]),
}

impl ObjectFit {
    pub const CENTER: [Rel; 2] = [(0, 500), (0, 500)];
    pub const CONTAIN: ObjectFit = ObjectFit::Contain(Self::CENTER);
    pub const COVER: ObjectFit = ObjectFit::Cover(Self::CENTER);
    pub const FILL: ObjectFit = ObjectFit::Fill(Self::CENTER);

    /// Where the image sits in its box.
    pub fn pos(self) -> [Rel; 2] {
        match self {
            Self::Contain(p) | Self::Cover(p) | Self::Fill(p) => p,
        }
    }

    /// The same fit placed at `pos`.
    pub fn at(self, pos: [Rel; 2]) -> Self {
        match self {
            Self::Contain(_) => Self::Contain(pos),
            Self::Cover(_) => Self::Cover(pos),
            Self::Fill(_) => Self::Fill(pos),
        }
    }
}
