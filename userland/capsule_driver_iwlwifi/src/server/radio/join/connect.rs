// NONOS Operating System (AGPL-3.0-or-later)

//! A connect: leave, hunt the network, run the join.

use crate::driver::Driver;
use crate::firmware::gen3::heard::hear;
use crate::firmware::gen3::join::hunt::{hunt, HuntEnd};
use crate::server::join_wire::{parse_connect, ConnectResult, CODE_BAD_REQUEST};
use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::rsn::JoinPolicy;

use super::super::clock::{now_ms, Uptime};
use super::super::{Radio, State};
use super::answers::{hidden, not_heard, refused};
use super::attempt::try_each;
use super::transmit::tx_region;

impl Radio {
    /// Join the network a connect body names.
    pub fn connect(&mut self, d: &Driver, body: &[u8]) -> ConnectResult {
        let Some(req) = parse_connect(body) else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let entropy = match self.join.ready(&req) {
            Ok(entropy) => entropy,
            Err(r) => return r,
        };
        self.disconnect();
        if req.hidden {
            return hidden();
        }
        let State::Up(up) = &mut self.state else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let Some(tx) = tx_region(&mut self.join, d) else {
            return ConnectResult::code(CODE_BAD_REQUEST);
        };
        let mut clock = Uptime;
        let policy = if req.wpa3_only { JoinPolicy::WPA3_ONLY } else { JoinPolicy::ANY };
        let mut found = Candidates::new(req.ssid, policy);
        let (results, beacons) = (&mut self.results, &mut self.beacons);
        let hunted = hunt(&mut up.dev, &mut clock, &mut self.sweep, &up.channels, &mut found, &mut now_ms, &mut |f| {
            if hear(results, f) {
                *beacons = beacons.saturating_add(1);
            }
        });
        // The background scan rests whatever the outcome.
        self.next_ms = now_ms().saturating_add(super::super::REST_MS);
        match (hunted, found.refusal()) {
            (Ok(()), _) => {}
            (Err(HuntEnd::NotHeard(_)), Some(why)) => return refused(why),
            (Err(HuntEnd::NotHeard(heard)), None) => return not_heard(heard),
            (Err(HuntEnd::Radio), _) => return ConnectResult::code(CODE_BAD_REQUEST),
        }
        try_each(up, &mut self.join, tx, &mut found, &req, policy, entropy)
    }
}
