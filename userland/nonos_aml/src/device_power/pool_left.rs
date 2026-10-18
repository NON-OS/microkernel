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

//! Giving power resources back for a device that has left: each count goes
//! down as for any release, and a resource whose last user it was is
//! turned off only while it is still there; one that went with the device
//! is only forgotten, since nothing is left to run its _OFF on.

use super::method::run;
use super::pool::PowerPool;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;

impl PowerPool {
    pub fn release_left(
        &mut self,
        aml: &mut Aml,
        eval: &mut Eval<'_>,
        res: &[Id],
        present: &dyn Fn(&Aml, Id) -> bool,
    ) -> Result<(), Fault> {
        let mut first = Ok(());
        for &r in res.iter().rev() {
            match self.held.get(&r).copied() {
                Some(1) => {
                    self.held.remove(&r);
                    if present(aml, r) {
                        first = first.and(run(aml, eval, r, *b"_OFF"));
                    }
                }
                Some(n) => {
                    self.held.insert(r, n - 1);
                }
                None => {}
            }
        }
        first
    }
}
