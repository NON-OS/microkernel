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

//! Power resources shared between devices (ACPI 6.5, 7.2): each is turned
//! on with _ON when the first device that needs it goes to D0, and off
//! with _OFF when the last one leaves. Turning one off under a device
//! still running would stop that device, so every use is counted.

use super::method::run;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;
use alloc::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct PowerPool {
    held: BTreeMap<Id, u32>,
}

impl PowerPool {
    pub fn holders(&self, res: Id) -> u32 {
        self.held.get(&res).copied().unwrap_or(0)
    }

    /// Takes each resource in order; on a failure, gives back what this
    /// call took, so the counts stay true.
    pub fn acquire(&mut self, aml: &mut Aml, eval: &mut Eval<'_>, res: &[Id]) -> Result<(), Fault> {
        for (i, &r) in res.iter().enumerate() {
            if self.holders(r) == 0 {
                if let Err(f) = run(aml, eval, r, *b"_ON_") {
                    let _ = self.release(aml, eval, &res[..i]);
                    return Err(f);
                }
            }
            *self.held.entry(r).or_insert(0) += 1;
        }
        Ok(())
    }

    /// Gives each resource back, last taken first; the first _OFF to fail
    /// is reported, and the rest still run.
    pub fn release(&mut self, aml: &mut Aml, eval: &mut Eval<'_>, res: &[Id]) -> Result<(), Fault> {
        let mut first = Ok(());
        for &r in res.iter().rev() {
            match self.held.get(&r).copied() {
                Some(1) => {
                    self.held.remove(&r);
                    let ran = run(aml, eval, r, *b"_OFF");
                    first = first.and(ran);
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
