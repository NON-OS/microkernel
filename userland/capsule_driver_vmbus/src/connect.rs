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

//! The contact and the offers. Versions are asked newest first until the
//! host takes one; then the offers are asked for and gathered until the
//! host says all are delivered.

use crate::constants::{
    ALLOFFERS_DELIVERED, CONNECTION_MESSAGE, CONNECTION_MESSAGE_V5, OFFERCHANNEL, OFFERS_MS,
    REPLY_MS, UNLOAD_RESPONSE, VERSIONS, VERSION_RESPONSE, VERSION_V5,
};
use crate::hv::Hv;
use crate::msg::{self, initiate_contact, kind, request_offers, unload, version_response};
use crate::offers::Offers;
use crate::wait;

/// The guest pages the contact names, as the host sees them.
#[derive(Clone, Copy)]
pub struct Pages {
    pub interrupt: u64,
    pub monitor1: u64,
    pub monitor2: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bus {
    pub version: u32,
    /// The connection channel messages go to.
    pub connection: u32,
}

/// The next message of `want`'s type within `ms`; others are dropped.
fn reply(hv: &Hv, want: u32, ms: u64) -> Option<crate::hv::Msg> {
    wait::until(ms, || hv.recv().filter(|m| kind(m.bytes()) == Some(want)))
}

pub fn connect(hv: &Hv, pages: Pages) -> Result<Bus, &'static str> {
    for &version in VERSIONS.iter() {
        let to = if version >= VERSION_V5 { CONNECTION_MESSAGE_V5 } else { CONNECTION_MESSAGE };
        hv.post(to, &initiate_contact(version, pages.interrupt, pages.monitor1, pages.monitor2))?;
        let m = reply(hv, VERSION_RESPONSE, REPLY_MS).ok_or("no answer to the contact")?;
        let (taken, named) = version_response(m.bytes()).ok_or("version answer cut short")?;
        if !taken {
            continue;
        }
        // From 5.0 the host names where messages go; the kernel lets the
        // core post on the two connections VMBus defines and no other.
        let connection = if version < VERSION_V5 {
            CONNECTION_MESSAGE
        } else if named == 0 {
            CONNECTION_MESSAGE_V5
        } else {
            named
        };
        if connection != CONNECTION_MESSAGE && connection != CONNECTION_MESSAGE_V5 {
            return Err("host named a message connection the kernel refuses");
        }
        return Ok(Bus { version, connection });
    }
    Err("no VMBus version in common")
}

/// Every offer until the host says all are delivered. A withdrawal among
/// them is answered and the offer forgotten.
pub fn offers(hv: &Hv, bus: Bus, out: &mut Offers) -> Result<(), &'static str> {
    hv.post(bus.connection, &request_offers())?;
    let done = wait::until(OFFERS_MS, || {
        let m = hv.recv()?;
        match kind(m.bytes()) {
            Some(ALLOFFERS_DELIVERED) => return Some(Ok(())),
            Some(OFFERCHANNEL) => match msg::offer(m.bytes()) {
                Some(o) => {
                    out.insert(o);
                }
                None => return Some(Err("offer cut short")),
            },
            Some(crate::constants::RESCIND_CHANNELOFFER) => {
                if let Err(e) = crate::serve::withdraw(hv, bus, out, m.bytes()) {
                    return Some(Err(e));
                }
            }
            _ => {}
        }
        None
    });
    done.unwrap_or(Err("offers not all delivered"))
}

/// Tell the host to forget the contact, for a core that gives up after it.
pub fn disconnect(hv: &Hv, bus: Bus) {
    if hv.post(bus.connection, &unload()).is_ok() {
        let _ = reply(hv, UNLOAD_RESPONSE, REPLY_MS);
    }
}
