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

use super::register_grant::RegisterGrant;
use crate::regs::Regs;

impl RegisterGrant {
    /// The legacy register window. A modern grant has none.
    pub fn regs(self) -> Option<Regs> {
        match self {
            Self::Mmio(g) => Some(Regs::mmio(g.user_va)),
            Self::Pio(g) => Some(Regs::pio(g.grant_id)),
            Self::Modern(_) => None,
        }
    }
}
