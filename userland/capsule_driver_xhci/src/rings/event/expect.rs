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
//! The armed interrupt TRBs whose events `park` keeps.
use super::issued_transfer::IssuedTransfer;
use super::park::Armed;
use super::state::EventRing;

impl EventRing {
    /// Whether a place is free for one more armed TRB.
    pub fn can_expect(&self) -> bool {
        self.can_expect_n(1)
    }
    /// Whether places are free for `n` more armed TRBs.
    pub fn can_expect_n(&self, n: usize) -> bool {
        self.armed.iter().filter(|a| a.is_none()).count() >= n
    }
    /// Keep a place for the event of `issued`, just armed. An endpoint has
    /// one TRB armed at a time, so a place it still held is stale and goes.
    pub fn expect(&mut self, issued: IssuedTransfer) -> bool {
        self.forget_parked(issued.slot, issued.dci);
        self.expect_also(issued)
    }
    /// Keep a place for `issued` beside those its endpoint holds: an isoch
    /// endpoint has a TD armed for each packet, two batches of them.
    pub fn expect_also(&mut self, issued: IssuedTransfer) -> bool {
        let Some(free) = self.armed.iter_mut().find(|a| a.is_none()) else { return false };
        *free = Some(Armed { issued, event: None });
        true
    }
    /// The TRB armed on `issued` completed, read by its own poll.
    pub fn settle(&mut self, issued: IssuedTransfer) {
        self.forget(|i| *i == issued);
    }
    /// Drop what is armed on endpoint `dci` of `slot`, when it is recovered:
    /// its TRBs were given up, and their addresses will be used again.
    pub fn forget_parked(&mut self, slot: u8, dci: u8) {
        self.forget(|i| i.slot == slot && i.dci == dci);
    }
    /// Drop what is armed on `slot`, once it is disabled: the slot id and
    /// its rings' frames may come back for another device.
    pub fn forget_slot(&mut self, slot: u8) {
        self.forget(|i| i.slot == slot);
    }
    fn forget(&mut self, gone: impl Fn(&IssuedTransfer) -> bool) {
        for a in self.armed.iter_mut() {
            if a.is_some_and(|a| gone(&a.issued)) {
                *a = None;
            }
        }
    }
}
