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

//! The controller's side: each command answered as its section says.

use super::events::connected;
use super::state::{Air, OURS};
use crate::hci_sim::controller::respond;
use crate::hci_sim::events::{complete, status};
use crate::hci_sim::link_events::disconnected;
use nonos_bt_hci::host::READ_RSSI;
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::hci::parse_command;
use nonos_bt_wire::l2cap::cid::ATT;

/// Remote User Terminated Connection, as a peer that is told to go says.
const ENDED_BY_US: u8 = 0x16;

impl Air {
    pub(super) fn command(&mut self, packet: &[u8]) {
        self.took(packet);
        let (opcode, p) = parse_command(packet).expect("a malformed command");
        let o = opcode.0;
        let events = match opcode {
            _ if self.lies.mute => Vec::new(),
            op::READ_BD_ADDR => vec![complete(1, o, 0, &OURS)],
            op::LE_SET_SCAN_ENABLE if p.first() == Some(&1) => {
                let mut heard = vec![complete(1, o, 0, &[])];
                heard.extend(self.others.iter().cloned());
                heard.extend(self.peer_heard());
                heard
            }
            op::LE_CREATE_CONNECTION => {
                self.new_link(p[12]);
                for pdu in std::mem::take(&mut self.early_att) {
                    self.send_to_host(ATT, &pdu);
                }
                vec![status(0, 1, o), connected(self.seen.from)]
            }
            op::LE_SET_SCAN_PARAMETERS => {
                self.seen.scan_filters.push(p[6]);
                respond(&self.cfg, packet)
            }
            op::LE_SET_RANDOM_ADDRESS => {
                self.seen.random = Some(p.try_into().expect("a random address of six"));
                respond(&self.cfg, packet)
            }
            op::LE_ENABLE_ENCRYPTION => self.encrypt(p),
            op::DISCONNECT if self.lies.forgot => vec![status(0x02, 1, o)],
            op::DISCONNECT if self.lies.linger => vec![status(0, 1, o)],
            op::DISCONNECT => {
                vec![status(0, 1, o), disconnected(u16::from_le_bytes([p[0], p[1]]), ENDED_BY_US)]
            }
            READ_RSSI => self.rssi(),
            op::LE_CONNECTION_UPDATE => vec![status(0, 1, o)],
            _ => respond(&self.cfg, packet),
        };
        for e in events {
            self.event(&e);
        }
    }
}
