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

//! Addressing a port's device on its slot and reading its configuration,
//! then configuring it by UAS (`uas_bind`) or by Bulk-Only (`bot_bind`).

use super::bot_bind::bot;
use super::probe::Probe;
use super::report::Stage;
use super::uas_bind::uas;
use crate::state::State;
use crate::xhci::{address_device, config_descriptor, E_BUSY};

pub(super) fn configure(xhci: u32, slot: u8, port: u8, state: &mut State) -> Probe {
    match address_device(xhci, slot, port) {
        Ok(()) => {}
        Err(E_BUSY) => return Probe::Busy,
        Err(e) => return Probe::Failed(Stage::Address, e),
    }
    let mut desc = [0u8; 255];
    let n = match config_descriptor(xhci, slot, &mut desc) {
        Ok(n) => n,
        Err(e) => return Probe::Failed(Stage::Config, e),
    };
    if let Some(probe) = uas(xhci, slot, port, &desc[..n]) {
        return probe;
    }
    bot(xhci, slot, port, &desc[..n], state)
}
