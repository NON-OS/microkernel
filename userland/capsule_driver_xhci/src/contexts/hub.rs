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

//! The input context of the Configure Endpoint that tells the controller a
//! slot is a hub (xHCI 1.2 sections 4.6.6 and 6.2.2.2, as recalled,
//! unverified: Evaluate Context does not take the hub fields). Only
//! A0 is set: the Slot Context is the controller's own, from the output
//! context, with the hub fields set, and every endpoint stays as it is.

use super::slot::{hub_dwords, HubInfo};
use super::slot_copy::{copy_slot_context, read_dw, write_dw};
use crate::dma::DmaRegion;

const ADD_SLOT: u32 = 1;
const INPUT_SLOT: usize = 1;
/// Context Entries stays at least EP0's.
const EP0_DCI: u8 = 1;

pub fn write_configure_hub_input(
    input: &DmaRegion,
    output: &DmaRegion,
    context_size: u8,
    hub: HubInfo,
    high_speed: bool,
) {
    input.zero();
    write_dw(input, context_size, 0, 1, ADD_SLOT);
    copy_slot_context(input, output, context_size, EP0_DCI);
    let dw = [0, 1, 2].map(|i| read_dw(input, context_size, INPUT_SLOT, i));
    for (i, value) in hub_dwords(dw, hub, high_speed).into_iter().enumerate() {
        write_dw(input, context_size, INPUT_SLOT, i, value);
    }
}
