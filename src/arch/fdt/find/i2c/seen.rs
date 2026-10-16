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

//! One node's properties as a phandle's target: a clock's fixed rate, an
//! interrupt controller's specifier width, and which controller it is by
//! its compatible, from the same lists the GIC and PLIC finders use.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::find::{gic, plic};
use crate::arch::fdt::property::Property;

use super::node::IrqCtrl;

const FIXED_CLOCK: &[u8] = b"fixed-clock";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Provider {
    pub interrupt_cells: Option<u32>,
    pub fixed_hz: Option<u32>,
    pub ctrl: IrqCtrl,
}

#[derive(Default)]
pub(super) struct Seen {
    pub phandle: Option<u32>,
    fixed: bool,
    hz: Option<u32>,
    interrupt_cells: Option<u32>,
    ctrl: IrqCtrl,
}

impl Seen {
    pub(super) fn see(&mut self, p: &Property) {
        match p.name {
            b"phandle" => self.phandle = be_u32(p.data, 0).ok(),
            b"compatible" => {
                self.fixed = p.compatible_matches(FIXED_CLOCK);
                self.ctrl = if gic::classify(p).is_some() {
                    IrqCtrl::Gic
                } else if plic::matches(p) {
                    IrqCtrl::Plic
                } else {
                    IrqCtrl::Other
                };
            }
            b"clock-frequency" => self.hz = be_u32(p.data, 0).ok(),
            b"#interrupt-cells" => self.interrupt_cells = be_u32(p.data, 0).ok(),
            _ => {}
        }
    }

    /// A clock's rate counts only from a fixed clock: on an I2C node the
    /// same property is the bus rate, and other clocks need their driver.
    pub(super) fn provider(self) -> Provider {
        let fixed_hz = self.hz.filter(|_| self.fixed);
        Provider { interrupt_cells: self.interrupt_cells, fixed_hz, ctrl: self.ctrl }
    }
}
