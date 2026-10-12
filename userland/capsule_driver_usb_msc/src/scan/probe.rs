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

//! Trying one root port: address its device, and bind it by UAS at high
//! speed (uas_bind.rs) or by Bulk-Only (bot_bind.rs).

use super::bot_bind::bot;
use super::report::Stage;
use super::uas_bind::uas;
use crate::disk::Disk;
use crate::state::State;
use crate::xhci::{address_device, config_descriptor, disable_slot, enable_slot, E_BUSY};

pub enum Probe {
    Ours(Disk),
    /// A device of another class, its first interface's class, subclass
    /// and protocol (`report::first_interface`); its slot is given back.
    NotOurs(u32),
    /// Another class driver is addressing the port; ask again later.
    Busy,
    /// The device did not answer as a BOT device should, at this stage,
    /// with this errno.
    Failed(Stage, i32),
}

pub fn probe_port(xhci: u32, port: u8, state: &mut State) -> Probe {
    let slot = match enable_slot(xhci) {
        Ok(slot) => slot,
        Err(e) => return Probe::Failed(Stage::Address, e),
    };
    let probe = bind(xhci, slot, port, state);
    if !matches!(probe, Probe::Ours(_)) {
        disable_slot(xhci, slot);
    }
    probe
}

fn bind(xhci: u32, slot: u8, port: u8, state: &mut State) -> Probe {
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
    if let Some(probe) = uas(xhci, slot, port, &desc[..n], state) {
        return probe;
    }
    bot(xhci, slot, &desc[..n], state)
}
