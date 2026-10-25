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

//! One node's properties as a phandle's target: whether it is a fixed
//! clock, and its rate.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::property::Property;

const FIXED_CLOCK: &[u8] = b"fixed-clock";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Provider {
    pub fixed_hz: Option<u32>,
}

#[derive(Default)]
pub(super) struct Seen {
    pub phandle: Option<u32>,
    fixed: bool,
    hz: Option<u32>,
}

impl Seen {
    pub(super) fn see(&mut self, p: &Property) {
        match p.name {
            b"phandle" => self.phandle = be_u32(p.data, 0).ok(),
            b"compatible" => self.fixed = p.compatible_matches(FIXED_CLOCK),
            b"clock-frequency" => self.hz = be_u32(p.data, 0).ok(),
            _ => {}
        }
    }

    /// A clock's rate counts only from a fixed clock: other clocks need
    /// their own driver to say what they run at.
    pub(super) fn provider(self) -> Provider {
        Provider { fixed_hz: self.hz.filter(|_| self.fixed) }
    }
}
