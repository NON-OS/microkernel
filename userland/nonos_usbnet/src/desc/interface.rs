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

//! The interfaces of a configuration, each alternate setting on its own,
//! with the bulk pipes it carries.

use alloc::vec::Vec;

use super::endpoint::endpoint;
use super::iface::Interface;
use super::kinds::{ENDPOINT, INTERFACE, SS_ENDPOINT_COMPANION};
use super::walk::walk;
use crate::bus::Pipes;

/// Every whole interface descriptor in `raw`, with the first bulk IN and
/// bulk OUT endpoint that follow it and their SuperSpeed bursts. One cut
/// short of its 9 bytes ends the interface before it, so the endpoints
/// after it are no interface's.
pub fn interfaces(raw: &[u8]) -> Vec<Interface> {
    let (mut out, mut open) = (Vec::<Interface>::new(), false);
    // The direction of the bulk endpoint just taken: a companion counts
    // only directly after its endpoint, and only whole (6 bytes, USB 3.2
    // table 9-27, as recalled, unverified).
    let mut last_in: Option<bool> = None;
    for d in walk(raw) {
        let before = last_in.take();
        match (d[1], out.last_mut()) {
            (INTERFACE, _) => {
                open = d.len() >= 9;
                if open {
                    out.push(Interface {
                        number: d[2],
                        alt: d[3],
                        class: d[5],
                        subclass: d[6],
                        protocol: d[7],
                        pipes: Pipes::default(),
                    });
                }
            }
            (ENDPOINT, Some(i)) if open && d.len() >= 7 => last_in = endpoint(i, d),
            (SS_ENDPOINT_COMPANION, Some(i)) if d.len() >= 6 => match before {
                Some(true) => i.pipes.burst_in = d[2] & 0x0F,
                Some(false) => i.pipes.burst_out = d[2] & 0x0F,
                None => {}
            },
            _ => {}
        }
    }
    out
}
