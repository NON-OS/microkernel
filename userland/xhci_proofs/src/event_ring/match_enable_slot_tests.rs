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

//! An Enable Slot answer naming no slot the table can hold: slot 0 is never
//! a device slot, and a slot past MaxSlots is refused by the table before
//! anything is indexed.

use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, TRB};
use crate::controller::issue_enable_slot;
use crate::error::XhciError;
use crate::slots::SlotTable;

#[test]
fn an_enable_slot_answer_naming_no_usable_slot_is_refused_before_any_table_is_indexed() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let first = cmd.crcr_value() & !0x3F;
    fx.hc.post(command(first, CC_SUCCESS, 0));
    let got = issue_enable_slot(fx.db(), fx.intr(), &mut cmd, &mut fx.ring);
    assert_eq!(got, Err(XhciError::ControllerUnsupported), "slot 0 is never a device slot");

    fx.hc.post(command(first + TRB, CC_SUCCESS, 255));
    let slot = issue_enable_slot(fx.db(), fx.intr(), &mut cmd, &mut fx.ring).expect("answered");
    assert_eq!(slot, 255);
    let max_slots = 8;
    let mut table = SlotTable::new();
    assert!(!table.mark_allocated(slot, max_slots), "past MaxSlots, the table refuses it");
    assert!(!table.is_allocated(slot, max_slots));
    assert!(table.resources_mut(slot, max_slots).is_none());
    assert!(table.mark_allocated(3, max_slots), "a slot inside MaxSlots is taken");
}
