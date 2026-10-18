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

//! An initiator before the first PDU: nothing agreed yet, nothing derived.

use super::identity::Identity;
use super::initiator::Initiator;
use super::state::State;
use super::step::Bond;
use crate::ecdh::KeyPair;
use crate::method::Method;
use crate::pdu::Pairing;

pub(super) struct Fresh {
    pub ours: Pairing,
    pub local: Option<Identity>,
    pub keys: KeyPair,
    pub a: [u8; 7],
    pub b: [u8; 7],
    pub now_ms: u64,
}

impl From<Fresh> for Initiator {
    fn from(f: Fresh) -> Self {
        Self {
            state: State::AwaitResponse,
            ours: f.ours,
            peer: None,
            keys: f.keys,
            method: Method::JustWorks,
            peer_x: [0; 32],
            dh_key: [0; 32],
            passkey: 0,
            na: [0; 16],
            nb: [0; 16],
            cb: [0; 16],
            a: f.a,
            b: f.b,
            mac_key: [0; 16],
            bond: Bond { ltk: [0; 16], irk: None, identity: None, authenticated: false },
            local: f.local,
            last_ms: f.now_ms,
        }
    }
}
