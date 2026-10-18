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


//! Which faults are the budget's rather than the method's: the caller may
//! treat a method cut short differently from one that failed on its own
//! (the event layer arms a cut GPE again, F-125).

use super::{Fault, FaultKind};

impl Fault {
    /// The evaluation ran out of steps, bytes or time.
    pub const fn is_budget(&self) -> bool {
        matches!(self.kind, FaultKind::Steps | FaultKind::Alloc | FaultKind::Time)
    }
}
