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
//! Joining a device another class driver configured and shares: the
//! slot it holds asked for, the descriptors read through it, and this
//! driver's binding shown the interfaces already held. Nothing is
//! addressed, configured or given back here: the device is the first
//! driver's, and the host frees its slot with its last holder.

use super::bound::Bound;
use super::probe::Probe;
use crate::bind::Bind;
use crate::found::{fetch, Found};
use crate::say::say_port;
use crate::xhci::{slot_of_port, XhciBus};

/// A driver's binding for a shared device: `claimed` has a bit set for
/// each interface another driver holds. It must not set a configuration.
pub type ShareFn<N> = fn(XhciBus, &Found, u32) -> Bind<N>;

pub(super) fn join<N>(xhci: u32, port: u8, tag: &[u8], share: ShareFn<N>) -> Probe<N> {
    // Claimed but not yet shared: its driver is still binding. Next pass.
    let Some((slot, claimed)) = slot_of_port(xhci, port) else { return Probe::Busy };
    let mut bus = XhciBus { xhci, slot };
    let found = match fetch(&mut bus, port) {
        Ok(found) => found,
        Err(e) => {
            say_port(tag, port, (0, 0), b"shared device unread", Some(e));
            return Probe::Failed;
        }
    };
    let ids = (found.info.vendor, found.info.product);
    match share(bus, &found, claimed) {
        Bind::Ours(nic) => {
            say_port(tag, port, ids, b"bound beside another driver", None);
            Probe::Ours(Bound { nic, xhci, slot, port })
        }
        Bind::NotOurs => Probe::NotOurs,
        Bind::Failed(what, e) => {
            say_port(tag, port, ids, what.as_bytes(), Some(e));
            Probe::Failed
        }
    }
}
