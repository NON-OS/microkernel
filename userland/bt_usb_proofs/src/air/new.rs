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

//! A quiet air: nothing advertised yet but `adv`, nothing scripted.

use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::seen::Seen;
use super::state::{Air, PEER};
use crate::hci_sim::config::Config;
use crate::hogp_sim::Peripheral;
use crate::smp_sim::Peer;
use nonos_bt_wire::l2cap::Reassembler;

impl Air {
    pub fn new(adv: &[u8], smp: Peer, hid: Peripheral, shown: Rc<Cell<Option<u32>>>) -> Self {
        Self {
            cfg: Config::default(),
            adv: adv.to_vec(),
            smp,
            hid,
            shown,
            script: VecDeque::new(),
            commands: Vec::new(),
            early_att: Vec::new(),
            att_answers: Vec::new(),
            signals: Vec::new(),
            ltk: None,
            refuse_encryption: false,
            seen: Seen { from: PEER, ..Seen::default() },
            lazy_credits: false,
            held: Vec::new(),
            pauses: 0,
            interrupt: VecDeque::new(),
            bulk_in: VecDeque::new(),
            joiner: Reassembler::new(512),
            idle: 0,
        }
    }
}
