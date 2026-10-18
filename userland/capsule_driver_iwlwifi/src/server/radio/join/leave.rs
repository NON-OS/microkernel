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
//! Leaving the network, and net_core's requests on the port.

use crate::firmware::gen3::join::fw::Fw;
use crate::firmware::gen3::join::link::Port;
use crate::firmware::gen3::join::run::leave;
use crate::server::join_wire::CODE_BAD_REQUEST;
use nonos_wifi_core::netif::{self};

use super::super::clock::Uptime;
use super::super::{Radio, State};
use super::say_link::say_link;
use super::state::Down;

impl Radio {
    /// Leave the network, telling the access point. 0, or -1 with the radio
    /// down.
    pub fn disconnect(&mut self) -> i32 {
        let State::Up(up) = &mut self.state else { return CODE_BAD_REQUEST };
        let (Some(tx), Some(mut j)) = (self.join.tx, self.join.joined.take()) else { return 0 };
        let mut clock = Uptime;
        let mut fw = Fw { dev: &mut up.dev, tx, clock: &mut clock, q: &mut self.join.q, inbox: &mut self.join.inbox };
        leave(&mut fw, &mut j);
        say_link(&j, &self.join.inbox, b"left the network");
        0
    }

    /// Answer one net_core link request into `out`.
    pub fn serve_link(&mut self, req: &[u8], out: &mut [u8]) -> Option<usize> {
        let (State::Up(up), Some(tx), Some(j)) = (&mut self.state, self.join.tx, self.join.joined.as_mut()) else {
            return netif::serve(req, &mut Down, out);
        };
        let mut clock = Uptime;
        let mut fw = Fw { dev: &mut up.dev, tx, clock: &mut clock, q: &mut self.join.q, inbox: &mut self.join.inbox };
        let mut port = Port { fw: &mut fw, link: &mut j.link };
        netif::serve(req, &mut port, out)
    }
}
