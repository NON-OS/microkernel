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
//! The open port serviced between requests.
use super::super::clock::{now_ms, Uptime};
use super::super::say::Line;
use super::super::{Radio, State};
use super::say_link::{say_link, say_loss};
use crate::firmware::gen3::join::fw::Fw;
use crate::firmware::gen3::join::link::Port;
use crate::firmware::gen3::join::run::forget;
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::start::stop_device;

impl Radio {
    /// Service the open port between requests.
    pub(in super::super) fn service_link(&mut self) {
        let (State::Up(up), Some(tx), Some(j)) = (&mut self.state, self.join.tx, self.join.joined.as_mut()) else {
            return;
        };
        let mut clock = Uptime;
        let mut fw = Fw { dev: &mut up.dev, tx, clock: &mut clock, q: &mut self.join.q, inbox: &mut self.join.inbox };
        Port { fw: &mut fw, link: &mut j.link }.service();
        if let Some((id, key)) = j.link.new_group_key.take() {
            if j.keys.rekey(&mut fw, &key, id).is_err() {
                Line::new().text(b"link: the new group key did not go in").send();
            }
        }
        if fw.dev.error_cause() {
            let m = fw.dev.m;
            stop_device(m, &mut Uptime);
            Line::new().text(b"radio down: ").text(Failure::Lost.text().as_bytes()).send();
            self.join.joined = None;
            self.state = State::Down(Failure::Lost);
            return;
        }
        if j.link.left.is_some() || j.link.silent {
            forget(&mut fw, j);
            say_loss(j.link.left);
            if let Some(j) = self.join.joined.take() {
                say_link(&j, &self.join.inbox, b"link closed");
            }
            self.next_ms = now_ms();
        }
    }
}
