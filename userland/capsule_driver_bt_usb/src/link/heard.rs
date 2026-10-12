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

//! The HID advertiser in one event, if the event is an LE Advertising
//! Report (Vol 4, Part E, 7.7.65.2) and one of its reports is one.

use super::find::Peer;
use super::pick::hid_advertiser;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::{LE_ADVERTISING_REPORT, LE_META};
use nonos_bt_wire::hci::le_meta::split;
use nonos_bt_wire::hci::reports::adv_reports;

pub(super) fn hid_in(event: &[u8]) -> Result<Option<Peer>, HciError> {
    let e = parse_event(event)?;
    if e.code != LE_META {
        return Ok(None);
    }
    let (code, sub) = split(e.params)?;
    if code != LE_ADVERTISING_REPORT {
        return Ok(None);
    }
    let found = adv_reports(sub)?.into_iter().find(hid_advertiser);
    Ok(found.map(|r| Peer { kind: r.addr_type, addr: r.addr }))
}
