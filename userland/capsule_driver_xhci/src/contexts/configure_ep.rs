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
use super::payload::Payload;
use super::slot_copy::{copy_slot_context, write_dw};
use crate::dma::DmaRegion;
const INPUT_CONTROL_INDEX: usize = 0;
const ADD_SLOT_FLAG: u32 = 1;
const EP_TYPE_INTERRUPT_IN: u32 = 7;
const CERR: u32 = 3;
const DCS: u32 = 1;
/// The parameters of the interrupt-IN endpoint being configured, bundled so the
/// writer stays within the argument limit. `interval` is already the
/// controller's exponent (`interrupt_interval`), not bInterval.
#[derive(Clone, Copy)]
pub struct EndpointConfig {
    pub context_size: u8,
    pub dci: u8,
    pub ring_phys: u64,
    pub payload: Payload,
    pub interval: u8,
}

/// The input context that adds one interrupt-IN endpoint, the Slot Context
/// taken from the device's `output` context.
pub fn write_configure_endpoint_input(input: &DmaRegion, output: &DmaRegion, cfg: EndpointConfig) {
    input.zero();
    write_dw(input, cfg.context_size, INPUT_CONTROL_INDEX, 1, ADD_SLOT_FLAG | (1 << cfg.dci));
    copy_slot_context(input, output, cfg.context_size, cfg.dci);
    write_endpoint(input, cfg);
}

/// Mult stays 0, as for every endpoint but a SuperSpeed isochronous one;
/// Max Burst Size and Max ESIT Payload carry the interval's every packet,
/// and the Average TRB Length is that payload, as Linux sets it for an
/// interrupt endpoint (xHCI 1.2 section 6.2.3).
fn write_endpoint(region: &DmaRegion, cfg: EndpointConfig) {
    let (csz, at, p) = (cfg.context_size, cfg.dci as usize + 1, cfg.payload);
    let dw0 = (cfg.interval as u32) << 16;
    let dw1 = (CERR << 1)
        | (EP_TYPE_INTERRUPT_IN << 3)
        | ((p.max_burst as u32) << 8)
        | ((p.max_packet as u32) << 16);
    write_dw(region, csz, at, 0, dw0);
    write_dw(region, csz, at, 1, dw1);
    write_dw(region, csz, at, 2, (cfg.ring_phys as u32) | DCS);
    write_dw(region, csz, at, 3, (cfg.ring_phys >> 32) as u32);
    write_dw(region, csz, at, 4, (p.esit as u32) | ((p.esit as u32) << 16));
}
