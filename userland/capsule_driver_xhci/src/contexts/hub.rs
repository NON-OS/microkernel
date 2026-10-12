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
//! Input contexts for a hub's slot: a routed device's Address Device input
//! gains its route and TT, and Configure Endpoint with only A0 set marks a
//! slot as a hub, its endpoints untouched (xHCI 1.2 sections 4.6.5 and
//! 4.6.6, as recalled, unverified).

use super::route::{as_hub, with_route, with_tt};
use super::slot_copy::{read_dw, write_dw};
use crate::dma::DmaRegion;

const ADD_SLOT: u32 = 1;
const INPUT_SLOT: usize = 1;
const OUTPUT_SLOT: usize = 0;
const SLOT_DWORDS: usize = 4;

/// Add the route and, for a device behind a high-speed hub, the TT to an
/// Address Device input `write_address_device_input` has written.
pub fn write_routed(input: &DmaRegion, context_size: u8, route: u32, tt: Option<(u8, u8)>) {
    let dw0 = read_dw(input, context_size, INPUT_SLOT, 0);
    write_dw(input, context_size, INPUT_SLOT, 0, with_route(dw0, route));
    if let Some((hub_slot, port)) = tt {
        let dw2 = read_dw(input, context_size, INPUT_SLOT, 2);
        write_dw(input, context_size, INPUT_SLOT, 2, with_tt(dw2, hub_slot, port));
    }
}

/// A Configure Endpoint input marking the slot whose context is `output`
/// as a hub; the rest of its Slot Context is handed back as it is.
pub fn write_hub_input(
    input: &DmaRegion,
    output: &DmaRegion,
    context_size: u8,
    hub: (u8, u8, bool),
) {
    input.zero();
    write_dw(input, context_size, 0, 1, ADD_SLOT);
    let mut dws = [0u32; SLOT_DWORDS];
    for (dw, v) in dws.iter_mut().enumerate() {
        *v = read_dw(output, context_size, OUTPUT_SLOT, dw);
    }
    let [a, b, c] = as_hub([dws[0], dws[1], dws[2]], hub.0, hub.1, hub.2);
    for (dw, v) in [a, b, c, dws[3]].into_iter().enumerate() {
        write_dw(input, context_size, INPUT_SLOT, dw, v);
    }
}
