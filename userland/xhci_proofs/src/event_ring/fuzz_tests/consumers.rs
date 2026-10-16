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

//! The ring's consumers that take no transfer of their own: the idle drain
//! and an Enable Slot command.

use super::driver::Driver;
use super::rng::Rng;
use crate::controller::drain_events::DRAIN_BATCH;
use crate::controller::{drain_events, issue_enable_slot};
use crate::error::XhciError;
use crate::event_ring::events::{command, CC_SUCCESS};
use crate::event_ring::fixture::{EP0_DCI, INT_DCI};
use crate::event_ring::producer::is_poison;

/// The slot table's size: an answer naming a slot past it is refused.
const MAX_SLOTS: u8 = 8;

impl Driver {
    pub(super) fn drain(&mut self, rng: &mut Rng) {
        let issued = self.hid.interrupt[0].armed.unwrap_or(0);
        self.noise(rng, issued, INT_DCI, 8);
        let batch = drain_events(self.fx.intr(), &mut self.fx.ring);
        assert!(batch.count <= DRAIN_BATCH);
        for t in &batch.trbs[..batch.count] {
            assert!(!is_poison(t), "the drain read past the segment");
        }
    }

    pub(super) fn enable_slot(&mut self, rng: &mut Rng) {
        let issued = self.next_command();
        self.noise(rng, issued, EP0_DCI, 0);
        let slot = rng.next() as u8;
        self.fx.hc.post(command(issued, CC_SUCCESS, slot));
        let got = issue_enable_slot(self.fx.db(), self.fx.intr(), &mut self.cmd, &mut self.fx.ring);
        self.cmd_issued += 1;
        match got {
            Ok(slot) => {
                assert_ne!(slot, 0, "slot 0 is never handed out");
                let taken = self.table.mark_allocated(slot, MAX_SLOTS);
                assert_eq!(taken, slot <= MAX_SLOTS, "slot {slot} against MaxSlots {MAX_SLOTS}");
                assert_eq!(self.table.mark_released(slot, MAX_SLOTS), taken);
            }
            Err(XhciError::ControllerUnsupported | XhciError::CommandCompletionFailed(_)) => {}
            Err(e) => panic!("enable slot failed: {e:?}"),
        }
    }
}
