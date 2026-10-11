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
//! A connect: leave, hunt the network, run the join.

use crate::driver::Driver;
use crate::firmware::gen3::heard::hear;
use crate::firmware::gen3::join::exchange::Progress;
use crate::firmware::gen3::join::fw::Fw;
use crate::firmware::gen3::join::hunt::{hunt, HuntEnd};
use crate::firmware::gen3::join::run::join;
use crate::server::join_wire::{parse_connect, ConnectResult, CODE_BAD_REQUEST, CODE_NO_ENTROPY};
use nonos_wifi_core::mlme::JoinRequest;
use nonos_wifi_core::rsn::JoinPolicy;

use super::super::clock::{now_ms, Uptime};
use super::super::entropy::draw_entropy;
use super::super::{Radio, State};
use super::answers::{hidden, not_heard};
use super::say::say_join;
use super::transmit::{recorded, tx_region};

impl Radio {
    /// Join the network a connect body names.
    pub fn connect(&mut self, d: &Driver, body: &[u8]) -> ConnectResult {
        let Some(req) = parse_connect(body) else { return ConnectResult::code(CODE_BAD_REQUEST) };
        self.disconnect();
        if req.hidden {
            return hidden();
        }
        let State::Up(up) = &mut self.state else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let Some(addr) = up.addr else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let Some(tx) = tx_region(&mut self.join, d) else {
            return ConnectResult::code(CODE_BAD_REQUEST);
        };
        let mut clock = Uptime;
        let (results, beacons) = (&mut self.results, &mut self.beacons);
        let found = hunt(&mut up.dev, &mut clock, &mut self.sweep, &up.channels, req.ssid, &mut now_ms, &mut |f| {
            if hear(results, f) {
                *beacons = beacons.saturating_add(1);
            }
        });
        // The background scan rests whatever the outcome.
        self.next_ms = now_ms().saturating_add(super::super::REST_MS);
        let (beacon, channel) = match found {
            Ok(f) => f,
            Err(HuntEnd::NotHeard(heard)) => return not_heard(heard),
            Err(HuntEnd::Radio) => return ConnectResult::code(CODE_BAD_REQUEST),
        };
        let Some(entropy) = draw_entropy() else { return ConnectResult::code(CODE_NO_ENTROPY) };
        let policy = if req.wpa3_only { JoinPolicy::WPA3_ONLY } else { JoinPolicy::ANY };
        let jr = JoinRequest { our_mac: addr, ssid: req.ssid, passphrase: req.pass, policy, entropy };
        let mut fw = Fw { dev: &mut up.dev, tx, clock: &mut clock, q: &mut self.join.q, inbox: &mut self.join.inbox };
        let mut progress = Progress::default();
        let out = join(&mut fw, &jr, &beacon, channel, up.tx_ant, up.rx_ant, &mut progress);
        say_join(&progress, &out);
        recorded(&mut self.join, req.ssid, out, progress)
    }
}
