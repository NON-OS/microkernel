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

//! A responder from Vol 3, Part H, 2.3.5.6, apart from the initiator under
//! test, with knobs to misbehave. `ab` are the link's two addresses: A, B.

use super::faults::Faults;
use super::rng::Rng;
use nonos_bt_smp::ecdh::KeyPair;
use nonos_bt_smp::method::{IoCap, Method};
use nonos_bt_smp::pdu::{auth, keys, Pairing};

/// The initiator's and responder's addresses in f5's form.
pub const A: [u8; 7] = [0, 0xc0, 0xde, 0x00, 0x11, 0x22, 0x33];
pub const B: [u8; 7] = [1, 0xd6, 0x5a, 0x44, 0x55, 0x66, 0x77];

pub struct Peer {
    pub ours: Pairing,
    pub keys: KeyPair,
    pub rng: Rng,
    pub passkey: u32,
    pub req: Option<Pairing>,
    pub method: Method,
    pub pka: [u8; 32],
    pub dh: [u8; 32],
    pub na: [u8; 16],
    pub nb: [u8; 16],
    pub ca: [u8; 16],
    pub round: u8,
    pub ltk: [u8; 16],
    pub irk: [u8; 16],
    pub identity: [u8; 7],
    pub ab: ([u8; 7], [u8; 7]),
    pub faults: Faults,
}

impl Peer {
    pub fn new(io_cap: IoCap, mitm: bool) -> Self {
        let auth_req = auth::BONDING | auth::SC | if mitm { auth::MITM } else { 0 };
        let (init_keys, resp_keys) = (0, keys::ID | keys::ENC);
        let ours = Pairing { io_cap, oob: false, auth_req, max_key_size: 16, init_keys, resp_keys };
        Self {
            ours,
            keys: KeyPair::from_secret([0x5c; 32]).unwrap(),
            rng: Rng(0x9e37_79b9_7f4a_7c15),
            passkey: 412_907,
            req: None,
            method: Method::JustWorks,
            pka: [0; 32],
            dh: [0; 32],
            na: [0; 16],
            nb: [0; 16],
            ca: [0; 16],
            round: 0,
            ltk: [0; 16],
            irk: *b"identity-key-irk",
            identity: [1, 0x10, 0x20, 0x30, 0x40, 0x50, 0xc0],
            ab: (A, B),
            faults: Faults::default(),
        }
    }
}
