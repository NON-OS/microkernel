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

//! Generated machines for the both-ways group proofs (H-03): sibling
//! functions on bus 0, and root ports with a switch, a conventional PCI
//! bridge or functions below; every function's ACS drawn from a seed.

use super::acs_tree::{Bdf, Func};
use super::symmetric_ports::below_port;
use super::symmetric_rng::Rng;

pub(super) struct Machine {
    pub funcs: Vec<Func>,
    pub acs: Vec<Option<(u16, u16)>>,
    /// The endpoints a driver may claim, by index into `funcs`.
    pub leaves: Vec<usize>,
}

impl Machine {
    pub(super) fn push(
        &mut self,
        r: &mut Rng,
        at: Bdf,
        buses: Option<(u8, u8)>,
        conv: bool,
        leaf: bool,
    ) {
        self.funcs.push(Func { at, buses, conventional: conv, alias: None });
        self.acs.push(r.acs());
        if leaf {
            self.leaves.push(self.funcs.len() - 1);
        }
    }
}

pub(super) fn machine(seed: u64) -> Machine {
    let mut r = Rng::seeded(seed);
    let mut m = Machine { funcs: Vec::new(), acs: Vec::new(), leaves: Vec::new() };
    for dev in 0..(1 + r.below(3)) as u8 {
        for f in 0..(1 + r.below(3)) as u8 {
            m.push(&mut r, (0, 0x10 + dev, f), None, false, true);
        }
    }
    let mut bus = 1u8;
    for port in 0..(1 + r.below(3)) as u8 {
        bus = below_port(&mut m, &mut r, port, bus) + 1;
    }
    m
}
