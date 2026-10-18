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

//! Ending armings: an endpoint recovered, or one transfer finished.

use super::issued_transfer::IssuedTransfer;
use super::parked::Parked;

impl Parked {
    /// Forget endpoint `dci` of `slot`: its arming and every event kept for
    /// it. Its transfers were given up, and their addresses will come again.
    pub fn forget(&mut self, slot: u8, dci: u8) {
        self.disarm(|a| a.slot == slot && a.dci == dci);
        for k in self.kept.iter_mut() {
            if k.is_some_and(|(_, e)| e.slot_id() == slot && e.endpoint_id() == dci) {
                *k = None;
            }
        }
    }

    pub(super) fn disarm(&mut self, which: impl Fn(&IssuedTransfer) -> bool) {
        for a in self.armed.iter_mut() {
            if a.is_some_and(|a| which(&a)) {
                *a = None;
            }
        }
    }
}
