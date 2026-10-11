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

//! The walk's parts: where a step leaves it, the challenge for the current
//! nonce, the next nonce, and what a walk that met the effort hands back:
//! the buyer key, and FETCH 1's request with the 36-byte proof-of-work field.

use nonos_nox_chunks::{pow_challenge, PowField, CHALLENGE_LEN};
use nonos_nox_config::ESCROW_POW_EFFORT;
use nonos_settle_proto::Escrow;

use super::walk::Walk;
use super::want::{Asked, Want};

pub const NONCE_TRIES: u32 = 256;

pub enum Walked {
    Going,
    Found(Asked),
    GaveUp,
}

impl Walk {
    pub fn want(&self) -> &Want {
        &self.want
    }

    pub(super) fn challenge(&self) -> [u8; CHALLENGE_LEN] {
        pow_challenge(&self.want.listing_key, &self.buyer_pk, &self.nonce, ESCROW_POW_EFFORT)
    }

    pub(super) fn next_nonce(&mut self) -> Walked {
        (self.hashx, self.tried) = (None, self.tried + 1);
        self.nonce = u128::from_le_bytes(self.nonce).wrapping_add(1).to_le_bytes();
        Walked::Going
    }

    pub(super) fn found(&self, solution: [u8; 16]) -> Asked {
        let field = PowField { nonce: self.nonce, effort: ESCROW_POW_EFFORT, solution };
        let w = &self.want;
        let request = Escrow {
            chain_id: w.chain_id,
            url: w.url.clone(),
            listing_key: w.listing_key,
            release_key: w.release_key,
            buyer_pk: self.buyer_pk,
            solution: field.encode().to_vec(),
        };
        Asked { buyer_sk: self.buyer_sk, request }
    }
}

/// A walk dropped mid-way, or after it found, leaves no buyer key behind.
impl Drop for Walk {
    fn drop(&mut self) {
        self.buyer_sk.fill(0);
    }
}
