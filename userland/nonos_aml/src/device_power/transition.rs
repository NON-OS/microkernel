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

//! Moving a device between D-states (ACPI 6.5, 7.3): the new state's
//! resources on (lowest ResourceOrder first), its _PSx, then the old
//! state's given back. A failed step undoes this move's own work.

use super::method::run;
use super::order::in_order;
use super::pool::PowerPool;
use super::rules::{refused, MoveError};
use super::state::DState;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::{listed, Eval};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Each device this has moved: its state and the resources it holds.
#[derive(Debug, Default)]
pub struct DevicePower {
    pub pool: PowerPool,
    held: BTreeMap<Id, (DState, Vec<Id>)>,
}

impl DevicePower {
    pub fn state(&self, dev: Id) -> Option<DState> {
        self.held.get(&dev).map(|(s, _)| s).copied()
    }

    fn needs(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, to: DState) -> Vec<Id> {
        let res = to.pr().map(|pr| listed(aml, eval, dev, pr)).unwrap_or_default();
        in_order(aml, res)
    }

    /// `from` is where the device is when this has not moved it before
    /// (from _PSC, or D0); afterwards this's own record is believed.
    pub fn set(
        &mut self,
        aml: &mut Aml,
        eval: &mut Eval<'_>,
        dev: Id,
        from: DState,
        to: DState,
    ) -> Result<(), MoveError> {
        let from = self.state(dev).unwrap_or(from);
        if from == to && self.held.contains_key(&dev) {
            return Ok(());
        }
        if let Some(e) = refused(aml, dev, from, to) {
            return Err(e);
        }
        let need = Self::needs(aml, eval, dev, to);
        self.pool.acquire(aml, eval, &need).map_err(MoveError::Fault)?;
        if let Err(f) = run(aml, eval, dev, to.ps()) {
            let _ = self.pool.release(aml, eval, &need);
            return Err(MoveError::Fault(f));
        }
        let old = self.held.insert(dev, (to, need)).map(|(_, r)| r).unwrap_or_default();
        self.pool.release(aml, eval, &old).map_err(MoveError::Fault)
    }
}
