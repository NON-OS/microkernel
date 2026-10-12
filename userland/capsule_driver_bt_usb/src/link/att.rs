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

//! The ATT channel. Before pairing ends, the peer may read this host's
//! small database (a keyboard reads the central's name); its requests are
//! answered from it and anything else is dropped. Once paired, the HID
//! host takes the channel (`nonos_bt_hogp`).

use super::env::Env;
use super::session::Session;
use nonos_bt_hogp::host::{Host as Hid, Outcome, Step};
use nonos_bt_hogp::server::serve;
use nonos_bt_wire::att::opcode::is_request;
use nonos_bt_wire::l2cap::cid::ATT;

impl Session {
    pub(super) fn start_hid<E: Env>(&mut self, env: &mut E) {
        let (hid, step) = Hid::start(env.now_ms());
        self.hid = Some(hid);
        self.hid_step(step, env);
    }

    pub(super) fn on_att<E: Env>(&mut self, pdu: &[u8], env: &mut E) {
        let now = env.now_ms();
        let step = match (self.hid.as_mut(), pdu.split_first()) {
            (Some(hid), _) => hid.on_pdu(pdu, now),
            (None, Some((&op, params))) if is_request(op) => Step::send(serve(op, params)),
            (None, _) => Step::default(),
        };
        self.hid_step(step, env);
    }

    pub(super) fn hid_step<E: Env>(&mut self, step: Step, env: &mut E) {
        for pdu in &step.send {
            self.send(ATT, pdu);
        }
        for input in step.input {
            self.post(input, env);
        }
        if let Some(b) = step.battery {
            env.battery(b);
        }
        match step.outcome {
            Some(Outcome::Ready { keyboard, mouse }) => {
                let k: &[u8] = if keyboard { b" keyboard" } else { b"" };
                let m: &[u8] = if mouse { b" mouse" } else { b"" };
                env.say(&[b"HID ready:", k, m]);
                env.ready(keyboard, mouse, self.may_type());
                if keyboard && !self.may_type() {
                    env.say(&[b"keys not posted: paired with no passkey typed"]);
                }
            }
            Some(Outcome::Failed(_)) => self.give_up("HID service unusable"),
            None => {}
        }
    }
}
