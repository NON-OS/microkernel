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

//! Ending a link this host gave up on: Disconnect (Vol 4, Part E, 7.1.6)
//! and its Disconnection Complete, waited for within the poll budget so
//! the next connection never meets the old one's event.

use nonos_bt_hci::host::POLL_BUDGET;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::event_code::DISCONNECTION_COMPLETE;
use nonos_bt_wire::hci::{opcode as op, DisconnectionComplete};

/// Remote User Terminated Connection (Vol 1, Part F, 2): the reason the
/// peer is given.
const USER_ENDED: u8 = 0x13;
/// Unknown Connection Identifier: the link was already gone.
const NO_SUCH_LINK: u8 = 0x02;

pub fn hang_up<T: Transport>(host: &mut Host<T>, handle: u16) -> Result<(), HciError> {
    let [h0, h1] = handle.to_le_bytes();
    match host.command_pending(op::DISCONNECT, &[h0, h1, USER_ENDED]) {
        Ok(()) => {}
        Err(HciError::Status(_, NO_SUCH_LINK)) => return Ok(()),
        Err(e) => return Err(e),
    }
    for _ in 0..POLL_BUDGET {
        match host.next_event()? {
            Some(event) if down(&event, handle) => return Ok(()),
            Some(_) => {}
            None => host.transport().pause(),
        }
    }
    Err(HciError::Timeout(op::DISCONNECT))
}

fn down(event: &[u8], handle: u16) -> bool {
    let Ok(e) = parse_event(event) else { return false };
    let d = (e.code == DISCONNECTION_COMPLETE).then(|| DisconnectionComplete::parse(e.params));
    matches!(d, Some(Ok(d)) if d.handle == handle)
}
