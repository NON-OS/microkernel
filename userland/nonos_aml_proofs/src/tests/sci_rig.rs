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

//! A q35 machine for the SCI proofs: the FADT's blocks over modelled
//! registers a GPE method's stand-in can look at while it runs.

use crate::events::{handlers, parse_fadt, Events, Policy};
use crate::fadt_enc::{q35, GPE0, PM1_STS};
use crate::load::Aml;
use crate::regs::{Regs, Shared};

pub use super::sci_gpes::gpes;
use std::collections::BTreeSet;

pub struct SciRig {
    pub regs: Shared,
    pub events: Events,
}

/// Status ports are write-one-to-clear: PM1 status, and GPE0's first half.
pub fn regs() -> Regs {
    let mut r = Regs::default();
    r.w1c.insert(PM1_STS);
    r.w1c.extend(GPE0..GPE0 + 8);
    r
}

/// Whether `gpe`'s status and enable bits are set.
pub fn bits(regs: &Shared, gpe: u16) -> (bool, bool) {
    let r = regs.0.borrow();
    let bit = 1 << (gpe % 8);
    (r.get(GPE0 + gpe / 8) & bit != 0, r.get(GPE0 + 8 + gpe / 8) & bit != 0)
}

impl SciRig {
    pub fn new(aml: &Aml, wake: &[u16], policy: Policy) -> Self {
        let f = parse_fadt(&q35(276)).unwrap();
        let wake: BTreeSet<u16> = wake.iter().copied().collect();
        let events = Events::new(&f, handlers(&aml.ns).0, &wake, policy).unwrap();
        SciRig { regs: Shared(std::rc::Rc::new(std::cell::RefCell::new(regs()))), events }
    }

    /// Raise `gpe`'s status bit, as its source does.
    pub fn raise(&self, gpe: u16) {
        let mut r = self.regs.0.borrow_mut();
        let v = r.get(GPE0 + gpe / 8) | (1 << (gpe % 8));
        r.set(GPE0 + gpe / 8, v);
    }

    pub fn start(&mut self) {
        let mut io = self.regs.clone();
        self.events.start(&mut io).unwrap();
    }

    pub fn pm1(&self, port: u16) -> u16 {
        self.regs.0.borrow().get(port)
    }

    pub fn set_pm1(&self, port: u16, v: u16) {
        self.regs.0.borrow_mut().set(port, v);
    }
}
