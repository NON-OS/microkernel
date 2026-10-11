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

//! Spending the budget, and the width integers are computed at.

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Width;

impl<H: Host> Machine<'_, H> {
    /// Counts `bytes` against the budget before they are allocated.
    pub(crate) fn charge(&mut self, bytes: u64, at: u32) -> Result<(), Fault> {
        let total = (self.alloc as u64).saturating_add(bytes);
        if total > self.limits.alloc as u64 {
            return Err(self.fault(FaultKind::Alloc, at));
        }
        self.alloc = total as usize;
        Ok(())
    }

    /// Counts `us` of waiting against the budget before it is spent.
    pub(crate) fn spend(&mut self, us: u64, at: u32) -> Result<(), Fault> {
        let total = self.time_us.saturating_add(us);
        if total > self.limits.time_ms.saturating_mul(1000) {
            return Err(self.fault(FaultKind::Time, at));
        }
        self.time_us = total;
        Ok(())
    }

    /// The DSDT's revision sets the integer width for every table (section
    /// 5.2.11.1); `_table` is kept so callers say which table they run.
    pub(crate) fn width(&self, aml: &Aml, _table: u16) -> Width {
        Width(aml.tables.first().is_none_or(|t| t.wide()))
    }
}
