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

use crate::arch::fdt::error::FdtError;

/// Why the tree's idle states were not taken. Any of these leaves every CPU
/// on a plain WFI, which needs nothing from the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    Tree(FdtError),
    /// More states than `MAX_STATES`.
    TooManyStates,
    /// A CPU naming more kept states than `MAX_PER_CPU`.
    TooManyForCpu,
}

impl From<FdtError> for Refusal {
    fn from(error: FdtError) -> Self {
        Self::Tree(error)
    }
}
