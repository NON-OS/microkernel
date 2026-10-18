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

//! Forgetting a device's runtime wake when the device has left: the
//! bookkeeping a disarm does, and none of its methods.

use super::pool::PowerPool;
use super::wake_types::{GpeSwitch, RuntimeWake};
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;

/// Whether `r` is declared somewhere below `dev` in the namespace.
fn below(aml: &Aml, mut r: Id, dev: Id) -> bool {
    for _ in 0..64 {
        let Some(p) = aml.ns.get(r).map(|e| e.parent).filter(|p| *p != r) else { return false };
        if p == dev {
            return true;
        }
        r = p;
    }
    false
}

impl RuntimeWake {
    /// The device has left: its event off with its last user and its
    /// resources given back, but nothing run on the device, which is gone,
    /// nor on a resource declared below it, which went with it.
    pub fn forget(
        &mut self,
        (pool, aml, eval): (&mut PowerPool, &mut Aml, &mut Eval<'_>),
        gpes: &mut dyn GpeSwitch,
        dev: Id,
    ) -> bool {
        let Some(a) = self.take(gpes, dev) else { return false };
        let _ = pool.release_left(aml, eval, &a.resources, &|aml, r| !below(aml, r, dev));
        true
    }
}
