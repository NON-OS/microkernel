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


//! The relays, the rendezvous point, the introduction points and the onion
//! service, answering what net.anon sends down its link.

use std::collections::{BTreeMap, VecDeque};
use std::vec::Vec;
use std::{format, vec};

use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use x25519_dalek::StaticSecret;

use crate::cell::{pack, unpack, Cell, Frame, RelayHeader, CELL_RELAY, CELL_RELAY_EARLY};
use crate::circuit::{open, seal, Hop};
use crate::link::Link;
use crate::path::{Flags, Relay};

use super::descriptor::Faults;
use super::keys::{self, Draw};
use super::ntor;
use super::service::Service;

const CELL_CREATE2: u8 = 10;
const CELL_CREATED2: u8 = 11;
const CELL_DESTROY: u8 = 4;
const BEGIN: u8 = 1;
const DATA: u8 = 2;
const END: u8 = 3;
const CONNECTED: u8 = 4;
const SENDME: u8 = 5;
const CIRCWINDOW_START: i32 = 1000;
const CIRCWINDOW_INCREMENT: i32 = 100;
const STREAMWINDOW_START: i32 = 500;
const STREAMWINDOW_INCREMENT: i32 = 50;
/// DESTROY reason PROTOCOL.
const REASON_PROTOCOL: u8 = 1;
const EXTEND2: u8 = 14;
const EXTENDED2: u8 = 15;
const BEGIN_DIR: u8 = 13;
const ESTABLISH_RENDEZVOUS: u8 = 33;
const INTRODUCE1: u8 = 34;
const RENDEZVOUS2: u8 = 37;
const RENDEZVOUS_ESTABLISHED: u8 = 39;
const INTRODUCE_ACK: u8 = 40;

/// One relay of the simulated network.
pub struct SimRelay {
    pub relay: Relay,
    pub onion: StaticSecret,
    /// For an HSDir: whether it holds the service's descriptor.
    pub holds: bool,
    /// For an introduction point: refuse with this status instead.
    pub refuse: Option<u16>,
    /// For an introduction point: which of the service's intro keys it carries.
    pub intro_index: Option<usize>,
    /// For an HSDir: answer 200 with something that is not a descriptor.
    pub garbage: bool,
    /// For an introduction point: whether the current descriptor lists it.
    pub listed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Who {
    Relay(usize),
    Service,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum StreamKind {
    Dir,
    Service,
}

struct SimCircuit {
    hops: Vec<Hop>,
    who: Vec<Who>,
    streams: BTreeMap<u16, StreamKind>,
    flow: Flow,
}

/// What a Tor exit keeps to hold the client to the windows (sendme.c,
/// relay.c), used when `World::strict_windows` is set.
#[derive(Default)]
struct Flow {
    /// Cells the circuit may still send: CIRCWINDOW_START, then +100 a SENDME.
    package: i32,
    /// Per stream: STREAMWINDOW_START, then +50 a SENDME.
    stream_package: BTreeMap<u16, i32>,
    /// The digest of every cell sent at a hundred boundary, oldest first,
    /// which each circuit SENDME must name in turn (sendme_record_cell_digest).
    recorded: VecDeque<[u8; 20]>,
    /// What is waiting for room: hop, command, stream, body.
    backlog: VecDeque<(usize, u8, u16, Vec<u8>)>,
}

pub struct World {
    pub relays: Vec<SimRelay>,
    pub service: Service,
    pub descriptor: Vec<u8>,
    /// What the service answers a request on its port with.
    pub service_reply: Vec<u8>,
    /// Whether the service goes to the rendezvous point after an introduction.
    pub meets: bool,
    /// Whether its RENDEZVOUS1 MAC is spoiled.
    pub bad_rendezvous_mac: bool,
    circuits: BTreeMap<u32, SimCircuit>,
    cookies: BTreeMap<[u8; 20], u32>,
    draw: Draw,
    /// Every relay body a relay (not the service) opened, for leak checks.
    pub relay_saw: Vec<Vec<u8>>,
    /// Every BEGIN body that reached the service.
    pub service_begins: Vec<Vec<u8>>,
    /// HSDirs asked, by relay index.
    pub hsdirs_asked: Vec<usize>,
    /// Intro points introduced through, by relay index.
    pub introduced_at: Vec<usize>,
    /// Circuits the client destroyed.
    pub destroyed: Vec<u32>,
    /// Requests the service received on a stream.
    pub service_requests: Vec<Vec<u8>>,
    /// For every INTRODUCE1, refused or not: the PoW effort it carried and
    /// whether the solution held, or `None` when it carried none.
    pub pow_seen: Vec<Option<(u32, bool)>>,
    pow_nonces: Vec<[u8; 16]>,
    /// Hold the client to the flow control windows the way a Tor exit does:
    /// stop at an empty window, and destroy the circuit (reason 1, PROTOCOL)
    /// on a circuit SENDME that is not version 1 naming the next recorded
    /// digest. Off, every answer is sent at once, as before.
    pub strict_windows: bool,
    /// Circuit and stream SENDMEs the exit accepted.
    pub circuit_sendmes: usize,
    pub stream_sendmes: usize,
    /// Circuits the exit destroyed over a SENDME it did not recognise.
    pub sendme_refused: Vec<u32>,
    /// DATA cells the exit has sent.
    pub data_cells_sent: usize,
}

impl World {
    pub fn new(relays: Vec<SimRelay>, service: Service, faults: Faults) -> Self {
        let points: Vec<(Vec<u8>, [u8; 32])> = {
            let mut listed: Vec<(usize, Vec<u8>, [u8; 32])> = relays
                .iter()
                .filter(|r| r.listed)
                .filter_map(|r| r.intro_index.map(|i| (i, specs(&r.relay), r.relay.ntor_onion_key)))
                .collect();
            listed.sort_by_key(|(i, _, _)| *i);
            listed.into_iter().map(|(_, s, k)| (s, k)).collect()
        };
        let descriptor = service.descriptor(&points, faults);
        Self {
            relays,
            service,
            descriptor,
            service_reply: b"HTTP/1.0 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\",\"lander\":\"anyone\"}".to_vec(),
            meets: true,
            bad_rendezvous_mac: false,
            circuits: BTreeMap::new(),
            cookies: BTreeMap::new(),
            draw: Draw::new("world-ephemeral"),
            relay_saw: Vec::new(),
            service_begins: Vec::new(),
            hsdirs_asked: Vec::new(),
            introduced_at: Vec::new(),
            destroyed: Vec::new(),
            service_requests: Vec::new(),
            pow_seen: Vec::new(),
            pow_nonces: Vec::new(),
            strict_windows: false,
            circuit_sendmes: 0,
            stream_sendmes: 0,
            sendme_refused: Vec::new(),
            data_cells_sent: 0,
        }
    }

    /// The service moves to the spare introduction points: the old ones
    /// refuse from now on and the HSDirs serve a new descriptor naming the
    /// spares.
    pub fn rotate(&mut self) {
        for r in self.relays.iter_mut() {
            if r.intro_index.is_some() && r.listed {
                r.listed = false;
                r.refuse = Some(2);
            } else if r.intro_index.is_none() && !r.listed {
                r.listed = true;
            }
        }
        let spares: Vec<usize> = (0..self.relays.len()).filter(|i| self.relays[*i].listed && self.relays[*i].intro_index.is_none() && !self.relays[*i].relay.flags.hsdir).collect();
        for (k, at) in spares.into_iter().enumerate() {
            self.relays[at].intro_index = Some(k);
        }
        let points: Vec<(Vec<u8>, [u8; 32])> = {
            let mut listed: Vec<(usize, Vec<u8>, [u8; 32])> = self
                .relays
                .iter()
                .filter(|r| r.listed)
                .filter_map(|r| r.intro_index.map(|i| (i, specs(&r.relay), r.relay.ntor_onion_key)))
                .collect();
            listed.sort_by_key(|(i, _, _)| *i);
            listed.into_iter().map(|(_, s, k)| (s, k)).collect()
        };
        self.descriptor = self.service.descriptor(&points, Faults::default());
    }

    /// Answer everything the client has sent since the last call.
    pub fn serve(&mut self, link: &mut Link) {
        let sent: Vec<Vec<u8>> = core::mem::take(&mut link.sent);
        for bytes in sent {
            let cell = Cell::decode(&bytes).expect("the client sends whole cells");
            for reply in self.cell(cell) {
                link.inbound.push_back(Frame::Fixed(alloc::boxed::Box::new(reply)));
            }
        }
    }

    fn by_rsa(&self, id: &[u8]) -> Option<usize> {
        self.relays.iter().position(|r| r.relay.rsa_identity[..] == *id)
    }

    fn cell(&mut self, mut cell: Cell) -> Vec<Cell> {
        match cell.command {
            CELL_CREATE2 => self.create2(&cell),
            CELL_DESTROY => {
                self.destroyed.push(cell.circuit);
                self.circuits.remove(&cell.circuit);
                Vec::new()
            }
            CELL_RELAY | CELL_RELAY_EARLY => {
                let Some(circuit) = self.circuits.get_mut(&cell.circuit) else { return Vec::new() };
                let opened = open(&mut circuit.hops, &mut cell.payload).expect("every client cell is for some hop");
                let header = unpack(&cell.payload);
                let body = crate::cell::body(&cell.payload).expect("length fits").to_vec();
                let who = circuit.who[opened.hop];
                if who != Who::Service {
                    self.relay_saw.push(body.clone());
                }
                self.relay_cell(cell.circuit, opened.hop, who, header, &body)
            }
            _ => Vec::new(),
        }
    }

    fn create2(&mut self, cell: &Cell) -> Vec<Cell> {
        let hlen = u16::from_be_bytes([cell.payload[2], cell.payload[3]]) as usize;
        let skin = &cell.payload[4..4 + hlen];
        let at = self.by_rsa(&skin[..20]).expect("CREATE2 names a known relay");
        let (y, _) = self.draw.x25519();
        let (reply, hop) = ntor::answer(skin, &self.relays[at].onion, &y);
        self.circuits.insert(cell.circuit, SimCircuit {
                hops: vec![hop],
                who: vec![Who::Relay(at)],
                streams: BTreeMap::new(),
                flow: Flow { package: CIRCWINDOW_START, ..Flow::default() },
            });
        let mut out = Cell::new(cell.circuit, CELL_CREATED2);
        out.payload[..2].copy_from_slice(&64u16.to_be_bytes());
        out.payload[2..66].copy_from_slice(&reply);
        vec![out]
    }

    /// A relay message from the client to hop `hop`.
    fn relay_cell(&mut self, id: u32, hop: usize, who: Who, header: RelayHeader, body: &[u8]) -> Vec<Cell> {
        match (header.command, who) {
            (EXTEND2, Who::Relay(_)) => self.extend2(id, hop, body),
            (BEGIN_DIR, Who::Relay(at)) => {
                assert!(self.relays[at].relay.flags.hsdir, "BEGIN_DIR only to an HSDir");
                assert!(body.is_empty(), "BEGIN_DIR carries no body");
                self.circuits.get_mut(&id).unwrap().streams.insert(header.stream, StreamKind::Dir);
                vec![self.reply(id, hop, CONNECTED, header.stream, &[])]
            }
            (DATA, Who::Relay(at)) => self.hsdir_request(id, hop, at, header.stream, body),
            (ESTABLISH_RENDEZVOUS, Who::Relay(_)) => {
                assert_eq!(body.len(), 20, "a rendezvous cookie is twenty bytes");
                self.cookies.insert(body.try_into().unwrap(), id);
                vec![self.reply(id, hop, RENDEZVOUS_ESTABLISHED, 0, &[])]
            }
            (INTRODUCE1, Who::Relay(at)) => self.introduce1(id, hop, at, body),
            (BEGIN, Who::Service) => {
                self.service_begins.push(body.to_vec());
                self.circuits.get_mut(&id).unwrap().streams.insert(header.stream, StreamKind::Service);
                vec![self.reply(id, hop, CONNECTED, header.stream, &[])]
            }
            (DATA, Who::Service) => {
                self.service_requests.push(body.to_vec());
                let reply = self.service_reply.clone();
                self.stream_out(id, hop, header.stream, &reply)
            }
            (SENDME, _) if self.strict_windows => self.sendme(id, header.stream, body),
            (BEGIN, Who::Relay(_)) => panic!("a BEGIN reached a relay: {:?}", core::str::from_utf8(body)),
            _ => Vec::new(),
        }
    }

    fn extend2(&mut self, id: u32, hop: usize, body: &[u8]) -> Vec<Cell> {
        let n = body[0] as usize;
        let (mut at, mut addr, mut rsa, mut ed) = (1usize, None, None, None);
        for _ in 0..n {
            let (kind, len) = (body[at], body[at + 1] as usize);
            let data = &body[at + 2..at + 2 + len];
            match kind {
                0 => addr = Some(data.to_vec()),
                2 => rsa = Some(data.to_vec()),
                3 => ed = Some(data.to_vec()),
                _ => {}
            }
            at += 2 + len;
        }
        let hlen = u16::from_be_bytes([body[at + 2], body[at + 3]]) as usize;
        let skin = body[at + 4..at + 4 + hlen].to_vec();
        let target = self.by_rsa(&rsa.expect("EXTEND2 names an RSA identity")).expect("EXTEND2 to a known relay");
        let relay = &self.relays[target].relay;
        let mut want_addr = relay.address.to_vec();
        want_addr.extend_from_slice(&relay.or_port.to_be_bytes());
        assert_eq!(addr.expect("EXTEND2 names an address"), want_addr);
        assert_eq!(ed.expect("EXTEND2 names an Ed25519 identity"), relay.ed25519_identity.to_vec());
        let (y, _) = self.draw.x25519();
        let (reply, new_hop) = ntor::answer(&skin, &self.relays[target].onion, &y);
        let circuit = self.circuits.get_mut(&id).unwrap();
        circuit.hops.push(new_hop);
        circuit.who.push(Who::Relay(target));
        let mut out = 64u16.to_be_bytes().to_vec();
        out.extend_from_slice(&reply);
        vec![self.reply(id, hop, EXTENDED2, 0, &out)]
    }

    fn hsdir_request(&mut self, id: u32, hop: usize, at: usize, stream: u16, body: &[u8]) -> Vec<Cell> {
        if self.circuits[&id].streams.get(&stream) != Some(&StreamKind::Dir) {
            return Vec::new();
        }
        self.hsdirs_asked.push(at);
        let want = format!("GET /tor/hs/3/{} HTTP/1.0\r\n\r\n", STANDARD_NO_PAD.encode(self.service.blinded.public));
        assert_eq!(core::str::from_utf8(body).unwrap(), want, "the HSDir request names the blinded key");
        let answer = if self.relays[at].garbage {
            b"HTTP/1.0 200 OK\r\n\r\nhs-descriptor 3\nthis is not a descriptor\n".to_vec()
        } else if self.relays[at].holds {
            let mut a = b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_vec();
            a.extend_from_slice(&self.descriptor);
            a
        } else {
            b"HTTP/1.0 404 Not found\r\n\r\n".to_vec()
        };
        self.stream_out(id, hop, stream, &answer)
    }

    fn introduce1(&mut self, id: u32, hop: usize, at: usize, body: &[u8]) -> Vec<Cell> {
        let index = self.relays[at].intro_index.expect("INTRODUCE1 only to an introduction point");
        self.introduced_at.push(at);
        /* The service's view, recorded even when this point refuses, so a
         * test can follow the effort from one introduction to the next. */
        let pow = self.service.introduce(index, body).expect("the service reads the introduction").pow;
        let seen = pow.map(|p| (p.effort, self.pow_holds(&p)));
        self.pow_seen.push(seen);
        if let Some(p) = pow {
            self.pow_nonces.push(p.nonce);
        }
        if let Some(status) = self.relays[at].refuse {
            return vec![self.reply(id, hop, INTRODUCE_ACK, 0, &[(status >> 8) as u8, status as u8, 0])];
        }
        let request = self.service.introduce(index, body).expect("the service accepts the introduction");
        let mut out = vec![self.reply(id, hop, INTRODUCE_ACK, 0, &[0, 0, 0])];
        if !self.meets {
            return out;
        }
        let rp_circuit = *self.cookies.get(&request.cookie).expect("the cookie names an established rendezvous point");
        let rp = match self.circuits[&rp_circuit].who[2] {
            Who::Relay(r) => r,
            Who::Service => unreachable!(),
        };
        assert_eq!(request.rp_onion_key, self.relays[rp].relay.ntor_onion_key, "the client named the rendezvous point's key");
        assert_eq!(request.rp_specs, specs(&self.relays[rp].relay), "and its link specifiers");
        let (y, _) = self.draw.x25519();
        let (handshake, keys) = self.service.rendezvous(index, &request, &y, self.bad_rendezvous_mac);
        out.push(self.reply(rp_circuit, 2, RENDEZVOUS2, 0, &handshake));
        let circuit = self.circuits.get_mut(&rp_circuit).unwrap();
        circuit.hops.push(ntor::service_hop(&keys));
        circuit.who.push(Who::Service);
        out
    }

    /// hs_pow_verify: the seed head names the service's seed, the nonce has
    /// not been seen, the Equi-X solution holds for the challenge, and
    /// BLAKE2b-32(challenge | solution) * effort fits in 32 bits. The effort
    /// check uses the blake2 crate, not the client's BLAKE2b; Equi-X is the
    /// nonos_equix verifier, which equix_proofs holds to the fork's own.
    fn pow_holds(&self, p: &super::service::PowField) -> bool {
        use blake2::digest::{Update as _, VariableOutput};
        let seed = super::descriptor::pow_seed();
        if p.seed_head != seed[..4] || self.pow_nonces.contains(&p.nonce) {
            return false;
        }
        let mut challenge = b"Tor hs intro v1\0".to_vec();
        challenge.extend_from_slice(&self.service.blinded.public);
        challenge.extend_from_slice(&seed);
        challenge.extend_from_slice(&p.nonce);
        challenge.extend_from_slice(&p.effort.to_be_bytes());
        if nonos_equix::verify(&challenge, &nonos_equix::Solution::from_bytes(&p.solution)).is_err() {
            return false;
        }
        let mut h = blake2::Blake2bVar::new(4).unwrap();
        h.update(&challenge);
        h.update(&p.solution);
        let mut r = [0u8; 4];
        h.finalize_variable(&mut r).unwrap();
        u64::from(u32::from_be_bytes(r)) * u64::from(p.effort) <= u64::from(u32::MAX)
    }

    /// `bytes` as DATA cells on `stream` from hop `hop`, then END DONE: all at
    /// once, or as the windows allow when they are enforced.
    fn stream_out(&mut self, id: u32, hop: usize, stream: u16, bytes: &[u8]) -> Vec<Cell> {
        if self.strict_windows {
            let flow = &mut self.circuits.get_mut(&id).unwrap().flow;
            flow.stream_package.entry(stream).or_insert(STREAMWINDOW_START);
            for piece in bytes.chunks(498) {
                flow.backlog.push_back((hop, DATA, stream, piece.to_vec()));
            }
            flow.backlog.push_back((hop, END, stream, vec![6]));
            return self.flush(id);
        }
        let mut out: Vec<Cell> = bytes.chunks(498).map(|piece| self.reply(id, hop, DATA, stream, piece)).collect();
        out.push(self.reply(id, hop, END, stream, &[6]));
        out
    }

    /// Send what the windows have room for, recording the digest of every
    /// DATA cell sent at a hundred boundary as relay.c does: the cell sent
    /// while the package window minus one is a multiple of the increment.
    fn flush(&mut self, id: u32) -> Vec<Cell> {
        let mut out = Vec::new();
        loop {
            let Some(circuit) = self.circuits.get_mut(&id) else { return out };
            let Some((hop, command, stream, body)) = circuit.flow.backlog.front().cloned() else {
                return out;
            };
            if command == DATA {
                let room = circuit.flow.stream_package.get(&stream).copied().unwrap_or(0);
                if circuit.flow.package <= 0 || room <= 0 {
                    return out;
                }
            }
            circuit.flow.backlog.pop_front();
            let window = circuit.flow.package;
            out.push(self.reply(id, hop, command, stream, &body));
            if command != DATA {
                continue;
            }
            self.data_cells_sent += 1;
            let circuit = self.circuits.get_mut(&id).unwrap();
            if (window - 1) % CIRCWINDOW_INCREMENT == 0 {
                let digest = circuit.hops[hop].forward_digest.peek();
                circuit.flow.recorded.push_back(digest);
            }
            circuit.flow.package -= 1;
            *circuit.flow.stream_package.get_mut(&stream).unwrap() -= 1;
        }
    }

    /// A SENDME from the client: a circuit one must be version 1 and name the
    /// oldest recorded digest (sendme_process_circuit_level), or the circuit
    /// is destroyed; a stream one grants that stream fifty more cells.
    fn sendme(&mut self, id: u32, stream: u16, body: &[u8]) -> Vec<Cell> {
        let Some(circuit) = self.circuits.get_mut(&id) else { return Vec::new() };
        if stream == 0 {
            let expected = circuit.flow.recorded.pop_front();
            let good = body.len() >= 23
                && body[0] == 1
                && u16::from_be_bytes([body[1], body[2]]) == 20
                && expected.is_some_and(|d| body[3..23] == d);
            if !good {
                self.sendme_refused.push(id);
                self.circuits.remove(&id);
                let mut destroy = Cell::new(id, CELL_DESTROY);
                destroy.payload[0] = REASON_PROTOCOL;
                return vec![destroy];
            }
            circuit.flow.package += CIRCWINDOW_INCREMENT;
            self.circuit_sendmes += 1;
        } else {
            *circuit.flow.stream_package.entry(stream).or_insert(0) += STREAMWINDOW_INCREMENT;
            self.stream_sendmes += 1;
        }
        self.flush(id)
    }

    /// A relay message from hop `hop` back to the client.
    fn reply(&mut self, id: u32, hop: usize, command: u8, stream: u16, body: &[u8]) -> Cell {
        let header = RelayHeader { command, recognized: 0, stream, length: body.len() as u16 };
        let mut payload = pack(&header, body).expect("fits one cell");
        seal(&mut self.circuits.get_mut(&id).unwrap().hops, hop, &mut payload).expect("hop exists");
        let mut cell = Cell::new(id, CELL_RELAY);
        cell.payload = payload;
        cell
    }
}

/// A relay's link specifiers as the descriptor and INTRODUCE1 write them.
pub fn specs(relay: &Relay) -> Vec<u8> {
    let mut out = vec![3u8, 0, 6];
    out.extend_from_slice(&relay.address);
    out.extend_from_slice(&relay.or_port.to_be_bytes());
    out.extend_from_slice(&[2, 20]);
    out.extend_from_slice(&relay.rsa_identity);
    out.extend_from_slice(&[3, 32]);
    out.extend_from_slice(&relay.ed25519_identity);
    out
}

/// A relay numbered `i`, every flag a lookup needs, on its own /16.
pub fn relay(i: u32) -> SimRelay {
    let (onion, ntor_key) = keys::x25519("relay-ntor", i);
    let ed = ed25519_dalek::SigningKey::from_bytes(&keys::bytes32("relay-ed", i)).verifying_key().to_bytes();
    let mut rsa = [0u8; 20];
    rsa.copy_from_slice(&keys::bytes32("relay-rsa", i)[..20]);
    SimRelay {
        relay: Relay {
            address: [10, i as u8, 0, 1],
            or_port: 9001,
            rsa_identity: rsa,
            ed25519_identity: ed,
            ntor_onion_key: ntor_key,
            flags: Flags {
                running: true,
                valid: true,
                fast: true,
                stable: true,
                guard: true,
                exit: false,
                authority: false,
                hsdir: true,
            },
            weight: 1000,
            exits_web: false,
        },
        onion,
        holds: false,
        refuse: None,
        intro_index: None,
        garbage: false,
        listed: true,
    }
}
