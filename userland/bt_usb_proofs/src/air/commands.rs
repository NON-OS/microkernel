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

//! The controller's side: each command packet answered as its section
//! says, with bring-up left to the HCI proofs' controller.

use super::events::{advertised, connected};
use super::state::{Air, HANDLE, OURS};
use crate::hci_sim::controller::respond;
use crate::hci_sim::events::{complete, status};
use crate::hci_sim::link_events::disconnected;
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::hci::parse_command;
use nonos_bt_wire::l2cap::cid::ATT;

/// Remote User Terminated Connection, as a peer that is told to go says.
const ENDED_BY_US: u8 = 0x16;

impl Air {
    pub(super) fn command(&mut self, packet: &[u8]) {
        self.commands.push(packet.to_vec());
        let (opcode, p) = parse_command(packet).expect("a malformed command");
        let o = opcode.0;
        let events = match opcode {
            op::READ_BD_ADDR => vec![complete(1, o, 0, &OURS)],
            op::LE_SET_SCAN_ENABLE if p.first() == Some(&1) => {
                vec![complete(1, o, 0, &[]), advertised(self.seen.from, 0x00, &self.adv)]
            }
            op::LE_CREATE_CONNECTION => {
                self.new_link(p[12]);
                for pdu in std::mem::take(&mut self.early_att) {
                    self.send_to_host(ATT, &pdu);
                }
                vec![status(0, 1, o), connected(self.seen.from)]
            }
            op::LE_SET_RANDOM_ADDRESS => {
                self.seen.random = Some(p.try_into().expect("a random address of six"));
                respond(&self.cfg, packet)
            }
            op::LE_ENABLE_ENCRYPTION => self.encrypt(p),
            op::DISCONNECT => vec![status(0, 1, o), disconnected(HANDLE, ENDED_BY_US)],
            op::LE_CONNECTION_UPDATE => vec![status(0, 1, o)],
            _ => respond(&self.cfg, packet),
        };
        for e in events {
            self.event(&e);
        }
    }
}
