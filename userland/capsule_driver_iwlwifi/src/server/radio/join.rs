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

//! Joining from the serving loop. A connect leaves any network first, maps
//! the transmit region on first use (one broker grant, kept while the
//! capsule lives once the firmware holds its addresses), hunts the network's
//! beacon, draws the join's randomness and runs the join
//! (`firmware::gen3::join`); the background scan rests while joined. Between
//! requests the loop services the open port: what the firmware received is
//! taken (the access point's rekeys answered and their keys installed), and
//! an association the access point ended, or a firmware that failed, closes
//! the port and takes the contexts down. net_core's link protocol is answered
//! from the open port, or as a link that is down.

use nonos_wifi_core::mlme::JoinRequest;
use nonos_wifi_core::netif::{self, LinkPort};
use nonos_wifi_core::rsn::JoinPolicy;

use crate::driver::Driver;
use crate::firmware::gen3::join::exchange::Progress;
use crate::firmware::gen3::join::fw::Fw;
use crate::firmware::gen3::join::hunt::{hunt, HuntEnd};
use crate::firmware::gen3::join::inbox::{Inbox, Queues};
use crate::firmware::gen3::join::link::Port;
use crate::firmware::gen3::join::run::{forget, join, leave, JoinEnd, Joined};
use crate::firmware::gen3::heard::hear;
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::start::stop_device;
use crate::firmware::gen3::txq::{TxQueue, TXQ_STRIDE, TX_REGION};
use crate::server::join_wire::{
    end_code, parse_connect, ConnectResult, LinkInfo, CODE_BAD_REQUEST, CODE_NOT_FOUND, CODE_NO_ENTROPY,
};

use super::clock::{now_ms, Uptime};
use super::entropy::draw_entropy;
use super::grant::Grant;
use super::say::Line;
use super::{Radio, State};

const SSID_MAX: usize = 32;

/// The radio's join state.
pub struct Join {
    tx: Option<&'static Grant>,
    q: Queues,
    inbox: Inbox,
    joined: Option<Joined>,
    ssid: [u8; SSID_MAX],
    ssid_len: usize,
}

impl Join {
    pub fn new() -> Self {
        Self {
            tx: None,
            q: Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) },
            inbox: Inbox::new(),
            joined: None,
            ssid: [0; SSID_MAX],
            ssid_len: 0,
        }
    }
}

/// net_core's view of a radio with no open port.
struct Down;

impl LinkPort for Down {
    fn mac(&self) -> Option<[u8; 6]> {
        None
    }
    fn link_up(&self) -> bool {
        false
    }
    fn poll_rx(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
    fn send_tx(&mut self, _frame: &[u8]) -> bool {
        false
    }
}

impl Radio {
    /// The radio runs joins: it is up, drew a station address and speaks
    /// every join command's layout.
    pub fn can_join(&self) -> bool {
        matches!(&self.state, State::Up(up) if up.addr.is_some() && up.join_api)
    }

    /// The port is open.
    pub fn joined(&self) -> bool {
        self.join.joined.as_ref().is_some_and(|j| j.link.station.is_associated())
    }

    /// The association, for the link reply.
    pub fn link_info(&self) -> Option<LinkInfo<'_>> {
        let j = self.join.joined.as_ref().filter(|j| j.link.station.is_associated())?;
        Some(LinkInfo {
            bssid: j.bss.target.bssid,
            ssid: &self.join.ssid[..self.join.ssid_len],
            akm: j.akm.suite_type(),
        })
    }

    /// Join the network a connect body names.
    pub fn connect(&mut self, d: &Driver, body: &[u8]) -> ConnectResult {
        let Some(req) = parse_connect(body) else { return ConnectResult::code(CODE_BAD_REQUEST) };
        self.disconnect();
        if req.hidden {
            // Its beacons carry no name, and this driver sends no probe.
            Line::new().text(b"join: a hidden network is not found by listening").send();
            return ConnectResult::code(CODE_NOT_FOUND);
        }
        let State::Up(up) = &mut self.state else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let Some(addr) = up.addr else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let tx = match self.join.tx {
            Some(t) => t,
            None => match Grant::map(d.device_id, d.claim_epoch, TX_REGION) {
                Some(g) => {
                    let t: &'static Grant = alloc::boxed::Box::leak(alloc::boxed::Box::new(g));
                    self.join.tx = Some(t);
                    t
                }
                None => {
                    Line::new().text(b"join: no DMA region for the transmit queues").send();
                    return ConnectResult::code(CODE_BAD_REQUEST);
                }
            },
        };
        let mut clock = Uptime;
        let (results, beacons) = (&mut self.results, &mut self.beacons);
        let found = hunt(&mut up.dev, &mut clock, &mut self.sweep, &up.channels, req.ssid, &mut now_ms, &mut |f| {
            if hear(results, f) {
                *beacons = beacons.saturating_add(1);
            }
        });
        // The background scan rests whatever the outcome.
        self.next_ms = now_ms().saturating_add(super::REST_MS);
        let (beacon, channel) = match found {
            Ok(f) => f,
            Err(HuntEnd::NotHeard(heard)) => {
                Line::new().text(b"join: network not heard among ").num(heard).text(b" beacons").send();
                let mut r = ConnectResult::code(CODE_NOT_FOUND);
                r.progress.recv = heard;
                return r;
            }
            Err(HuntEnd::Radio) => return ConnectResult::code(CODE_BAD_REQUEST),
        };
        let Some(entropy) = draw_entropy() else { return ConnectResult::code(CODE_NO_ENTROPY) };
        let policy = if req.wpa3_only { JoinPolicy::WPA3_ONLY } else { JoinPolicy::ANY };
        let jr = JoinRequest { our_mac: addr, ssid: req.ssid, passphrase: req.pass, policy, entropy };
        let mut fw = Fw { dev: &mut up.dev, tx, clock: &mut clock, q: &mut self.join.q, inbox: &mut self.join.inbox };
        let mut progress = Progress::default();
        let out = join(&mut fw, &jr, &beacon, channel, up.tx_ant, up.rx_ant, &mut progress);
        say_join(&progress, &out);
        let mut r = ConnectResult { progress, state: progress.state, ..ConnectResult::default() };
        match out {
            Ok(j) => {
                r.akm = j.akm.suite_type();
                self.join.ssid[..req.ssid.len()].copy_from_slice(req.ssid);
                self.join.ssid_len = req.ssid.len();
                self.join.joined = Some(j);
            }
            Err(e) => (r.code, r.ap_code) = end_code(e),
        }
        r
    }

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

    /// Service the open port between requests.
    pub(super) fn service_link(&mut self) {
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
        if let Some(reason) = j.link.left {
            forget(&mut fw, j);
            Line::new().text(b"link: the access point ended the association, reason ").num(u32::from(reason)).send();
            if let Some(j) = self.join.joined.take() {
                say_link(&j, &self.join.inbox, b"link closed");
            }
            self.next_ms = now_ms();
        }
    }
}

// One line on how a join went.
fn say_join(p: &Progress, out: &Result<Joined, JoinEnd>) {
    let mut l = Line::new();
    match out {
        Ok(_) => l.text(b"join: port open"),
        Err(JoinEnd::Refused(f)) => {
            let (code, ap) = crate::server::join_wire::failure_code(*f);
            l.text(b"join: refused, code ").num(code.unsigned_abs()).text(b" status ").num(u32::from(ap))
        }
        Err(JoinEnd::TimedOut) => l.text(b"join: the access point stopped answering"),
        Err(JoinEnd::NotTheNetwork) => l.text(b"join: the beacon names no usable channel"),
        Err(JoinEnd::Setup(e)) => l.text(b"join: a firmware context did not go up: ").text(setup_text(e)),
        Err(JoinEnd::Firmware(Some(c))) => l.text(b"join: command ").hex(u32::from(c.group) << 8 | u32::from(c.cmd)).text(b" failed"),
        Err(JoinEnd::Firmware(None)) => l.text(b"join: the firmware raised its error cause"),
        Err(JoinEnd::Keys(_)) => l.text(b"join: a key did not go into the firmware"),
    };
    l.text(b"; sent ").num(p.sent).text(b" recv ").num(p.recv).text(b" eapol ").num(p.eapol);
    l.text(b" state ").num(u32::from(p.state)).text(b" refused ").num(p.refused).send();
}

fn setup_text(e: &crate::firmware::gen3::join::bss::SetupError) -> &'static [u8] {
    use crate::firmware::gen3::join::bss::SetupError;
    use crate::firmware::gen3::station::session::Session;
    match e {
        SetupError::Command(c) if c.why == crate::firmware::gen3::dev::WaitError::FirmwareError => b"firmware error",
        SetupError::Command(_) => b"a command went unanswered",
        SetupError::BadQueueReply => b"a malformed queue reply",
        SetupError::NoSession(Some(Session::Refused)) => b"session protection refused",
        SetupError::NoSession(_) => b"never on channel",
        SetupError::Region => b"the transmit region",
    }
}

// One line of the link's counts when it closes.
fn say_link(j: &Joined, inbox: &Inbox, what: &[u8]) {
    let s = &j.link.stats;
    Line::new()
        .text(what)
        .text(b": tx ")
        .num(s.tx_ok)
        .text(b"/")
        .num(s.tx_drop)
        .text(b" rx ")
        .num(s.rx_eth)
        .text(b"/")
        .num(s.rx_frames)
        .text(b" refused ")
        .num(s.rx_refused)
        .text(b" full ")
        .num(s.rx_full)
        .text(b" rekeys ")
        .num(s.rekeys)
        .text(b" dropped ")
        .num(inbox.dropped)
        .text(b" stray ")
        .num(inbox.stray)
        .text(b" failed ")
        .num(inbox.tx_failed)
        .send();
}
