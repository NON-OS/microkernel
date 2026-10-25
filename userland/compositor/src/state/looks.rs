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

//! How each output is shown beyond its scale: night light (one colour
//! temperature for every output), each output's accessibility settings, the
//! looks built from the two, and each magnified output's lens this frame.

use alloc::boxed::Box;

use crate::display::access_rule::Access;
use crate::display::layout_rule::MAX_SLICES;
use crate::display::magnifier_rule::Lens;
use crate::frame_pacer::look::{build, Look};

#[derive(Default)]
pub struct Looks {
    /// Night light's ramps while the shell has it on (None: true colour).
    pub night: Option<Box<[[u8; 256]; 3]>>,
    pub access: [Access; MAX_SLICES],
    pub looks: [Option<Box<Look>>; MAX_SLICES],
    pub lens: [Option<Lens>; MAX_SLICES],
}

impl Looks {
    /// Build every output's look again after night light or a setting moved.
    pub fn rebuild(&mut self) {
        let night = self.night.as_deref();
        for (look, a) in self.looks.iter_mut().zip(self.access.iter()) {
            *look = build(a, night);
        }
    }
}
