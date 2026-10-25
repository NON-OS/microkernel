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

//! The initiator's side of LE Secure Connections pairing (Vol 3, Part H,
//! 2.3.5.6), as a state machine fed with the peer's SMP PDUs, the user's
//! answers and the time. It sends nothing itself: each call returns the
//! PDUs for the caller to send on the SMP fixed channel, CID 0x0006
//! (Vol 3, Part A, 2.1). Nonces, keys and confirm values are held most
//! significant octet first, as the functions take them.

use super::identity::Identity;
use super::state::State;
use super::step::Bond;
use crate::ecdh::KeyPair;
use crate::method::Method;
use crate::pdu::Pairing;

/// Passkey entry runs one round per passkey bit (Vol 3, Part H, 2.3.5.6.3).
pub const ROUNDS: u8 = 20;

/// The SMP timer: pairing fails when no PDU has been sent or received for
/// 30 s, and no SMP PDU may follow on that link (Vol 3, Part H, 3.4).
pub const TIMEOUT_MS: u64 = 30_000;

pub struct Initiator {
    pub(super) state: State,
    pub(super) ours: Pairing,
    /// The Pairing Response, once it has come.
    pub(super) peer: Option<Pairing>,
    pub(super) keys: KeyPair,
    /// Chosen from both sides' capabilities when the response comes.
    pub(super) method: Method,
    pub(super) peer_x: [u8; 32],
    pub(super) dh_key: [u8; 32],
    pub(super) passkey: u32,
    pub(super) na: [u8; 16],
    pub(super) nb: [u8; 16],
    /// The responder's confirm value for the current round.
    pub(super) cb: [u8; 16],
    /// Our address and the peer's, as f5 and f6 take them.
    pub(super) a: [u8; 7],
    pub(super) b: [u8; 7],
    pub(super) mac_key: [u8; 16],
    pub(super) bond: Bond,
    /// This host's identity, sent if the responder asks for it.
    pub(super) local: Option<Identity>,
    pub(super) last_ms: u64,
}
