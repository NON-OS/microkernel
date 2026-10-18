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

//! The first publish pass (L55), as steps that must all be done before
//! init is told: the namespace loaded with `_INI` run, the host bridges'
//! `_OSC`, the hidden functions given `_PS0` and probed (or their budget
//! spent), and the platform records published. The flag goes up once,
//! never before the publish, and never again. Pure: `capsule_acpi_proofs`
//! mounts it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Namespace = 1,
    Osc = 2,
    Hidden = 4,
    Records = 8,
}

const ALL: u8 = 15;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FirstPass {
    done: u8,
    raised: bool,
}

impl FirstPass {
    pub fn finish(&mut self, s: Step) {
        self.done |= s as u8;
    }

    /// True exactly once: the first time every step is done.
    pub fn raise(&mut self) -> bool {
        let now = !self.raised && self.done == ALL;
        self.raised |= now;
        now
    }
}
