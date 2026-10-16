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

//! What one event says of the advertisers in range: from an LE Advertising
//! Report (Vol 4, Part E, 7.7.65.2), a bonded peer that may be connected
//! to, else the first new HID advertiser; from an LE Directed Advertising
//! Report (7.7.65.11), a bonded peer aimed at one of this host's private
//! addresses.

use super::aimed::directed;
use super::env::Env;
use super::find::Peer;
use super::pick::{connectable, hid_advertiser};
use super::us::Us;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::{
    LE_ADVERTISING_REPORT, LE_DIRECTED_ADVERTISING_REPORT, LE_META,
};
use nonos_bt_wire::hci::le_meta::split;
use nonos_bt_wire::hci::reports::adv_reports;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Heard {
    /// Found by identity or by the IRK that resolves its address, from a
    /// connectable report of either kind: the bond was made over a HID
    /// link, so what it advertises does not matter.
    Bonded(Peer),
    New(Peer),
}

pub(super) fn heard_in<E: Env>(
    event: &[u8],
    env: &mut E,
    us: &Us,
) -> Result<Option<Heard>, HciError> {
    let e = parse_event(event)?;
    if e.code != LE_META {
        return Ok(None);
    }
    let (code, sub) = split(e.params)?;
    if code == LE_DIRECTED_ADVERTISING_REPORT {
        return Ok(directed(sub, env, us)?.map(Heard::Bonded));
    }
    if code != LE_ADVERTISING_REPORT {
        return Ok(None);
    }
    // A device with no bond here pairs only while the user lets one in.
    let may_pair = env.may_pair();
    let mut new = None;
    for r in adv_reports(sub)? {
        let peer = Peer { kind: r.addr_type, addr: r.addr };
        if connectable(&r) && env.bonded(r.addr_type, &r.addr).is_some() {
            return Ok(Some(Heard::Bonded(peer)));
        }
        if may_pair && new.is_none() && hid_advertiser(&r) {
            new = Some(Heard::New(peer));
        }
    }
    Ok(new)
}
