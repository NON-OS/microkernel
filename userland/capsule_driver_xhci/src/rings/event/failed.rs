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
use super::state::EventRing;
use crate::regs::op::controller_failed;
impl EventRing {
    pub(crate) fn set_op_base(&mut self, op_base: u64) {
        self.op_base = op_base;
    }
    /// Whether the controller this ring was given to has failed, so a wait
    /// on it ends now rather than at its deadline; a ring given to none
    /// has not.
    pub fn controller_failed(&self) -> bool {
        self.op_base != 0 && controller_failed(self.op_base)
    }
}
