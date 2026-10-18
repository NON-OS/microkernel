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

//! One pairing between the initiator under test and the simulated
//! responder, PDU by PDU, with the users played as the test asks.

use super::peer::{Peer, A, B};
use super::rng::Rng;
use nonos_bt_smp::pairing::{Identity, Initiator, Outcome, User, REQUEST};
use nonos_bt_smp::pdu::Pairing;
use std::collections::VecDeque;

pub struct Run {
    pub init: Initiator,
    pub peer: Peer,
    pub rng: Rng,
    pub now: u64,
    /// Every PDU the initiator sent, the Pairing Request first.
    pub sent: Vec<Vec<u8>>,
    pub users: Vec<User>,
    pub outcomes: Vec<Outcome>,
    /// The numeric comparison answer, if not an honest one.
    pub agree: Option<bool>,
    /// The passkey typed, if not the one the responder shows.
    pub typed: Option<Option<u32>>,
    pub(super) to_peer: VecDeque<Vec<u8>>,
    pub(super) to_init: VecDeque<Vec<u8>>,
}

impl Run {
    pub fn new(peer: Peer) -> Self {
        Self::with(peer, REQUEST, None)
    }

    /// A run whose initiator offers `ours`, with `local` as its identity.
    pub fn with(peer: Peer, ours: Pairing, local: Option<Identity>) -> Self {
        let mut rng = Rng(0x2545_f491_4f6c_dd1d);
        let (init, request) = Initiator::start_private(ours, local, A, B, &mut rng, 0).unwrap();
        let (users, outcomes, to_init) = (Vec::new(), Vec::new(), VecDeque::new());
        let to_peer = VecDeque::from([request.clone()]);
        let (agree, typed) = (None, None);
        Self {
            init,
            peer,
            rng,
            now: 0,
            sent: vec![request],
            users,
            outcomes,
            agree,
            typed,
            to_peer,
            to_init,
        }
    }
}
