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

//! Trying one root port: address its device, read what it offers, and ask
//! the driver whether it is its own. The slot is given back unless it is.

use super::bound::Bound;
use super::classify::classify;
use crate::run::BindFn;
use crate::say::say_port;
use crate::xhci::{disable_slot, enable_slot, XhciBus};

pub(super) enum Probe<N> {
    Ours(Bound<N>),
    NotOurs,
    /// Another class driver is addressing the port; ask again next pass.
    Busy,
    Failed,
}

pub(super) fn probe<N>(xhci: u32, port: u8, tag: &[u8], bind: BindFn<N>) -> Probe<N> {
    let slot = match enable_slot(xhci) {
        Ok(slot) => slot,
        Err(e) => {
            say_port(tag, port, (0, 0), b"no device slot", Some(e));
            return Probe::Failed;
        }
    };
    let out = classify(XhciBus { xhci, slot }, port, tag, bind);
    if !matches!(out, Probe::Ours(_)) {
        disable_slot(xhci, slot);
    }
    out
}
