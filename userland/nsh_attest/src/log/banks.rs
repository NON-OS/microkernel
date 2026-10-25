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

//! The banks a log's header declares.

use super::MAX_BANKS;

/// The banks the header declares, `(algorithm, digest size)`, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Banks {
    pub algs: [(u16, u16); MAX_BANKS],
    pub n: usize,
}

impl Banks {
    pub fn size_of(&self, alg: u16) -> Option<usize> {
        self.algs.get(..self.n)?.iter().find(|a| a.0 == alg).map(|a| a.1 as usize)
    }
}
