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

//! Taking one root port's device: a slot for it, configured (`configure`),
//! and given back unless the device is ours to ask. Its logical units are
//! asked after, a command a step (`ask`).

use super::asking::Asking;
use super::configure::configure;
use super::report::Stage;
use crate::state::State;
use crate::xhci::{disable_slot, enable_slot};

pub enum Probe {
    /// Configured: its logical units are to be asked.
    Bound(Asking),
    /// A device of another class, its first interface's class, subclass
    /// and protocol (`report::first_interface`); its slot is given back.
    NotOurs(u32),
    /// Another class driver is addressing the port; ask again later.
    Busy,
    /// The device did not answer as a BOT device should, at this stage,
    /// with this errno.
    Failed(Stage, i32),
}

pub fn bind(xhci: u32, port: u8, state: &mut State) -> Probe {
    let slot = match enable_slot(xhci) {
        Ok(slot) => slot,
        Err(e) => return Probe::Failed(Stage::Address, e),
    };
    let probe = configure(xhci, slot, port, state);
    if !matches!(probe, Probe::Bound(_)) {
        disable_slot(xhci, slot);
    }
    probe
}
