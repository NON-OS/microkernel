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

//! The sealed answer's names: the injected X25519, why an answer
//! does not open, and its key.

use nonos_marketplace_abi::Keccak;

pub const ANSWER_TAG: &[u8] = b"NONOS.nox.escrow.answer.v1";

/// X25519, injected as keccak is: the meter and the agent bring their own.
pub trait X25519 {
    fn dh(&self, secret: &[u8; 32], public: &[u8; 32]) -> [u8; 32];
    fn public(&self, secret: &[u8; 32]) -> [u8; 32];
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnswerWrong {
    /// Shorter than agent_pk and a tag.
    Short,
    /// agent_pk makes an all-zero shared secret.
    LowOrder,
    /// The tag does not verify under buyer_sk.
    Seal,
    /// It opened, but is not exactly 64 + 1056 n bytes for the release's
    /// content_len.
    Body,
}

type Keys<'a> = (&'a [u8; 32], &'a [u8; 32]);

pub(crate) fn answer_key(k: &impl Keccak, shared: &[u8; 32], (agent, buyer): Keys<'_>) -> [u8; 32] {
    k.keccak256(&[ANSWER_TAG, shared, agent, buyer].concat())
}
