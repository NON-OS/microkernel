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

//! Starting a session: the grant is checked to be a base grant for the
//! user's own key, a fresh X-Wing key pair is made from the caller's seed,
//! and `CLIENT_HELLO` is queued. Nothing else is sent until the server has
//! answered and its host has been checked.

use super::stage::Stage;
use super::Client;
use super::Config;
use crate::framer::Framer;
use crate::record::Bound;
use crate::{Attester, Refusal, UserSigner};
use alloc::vec::Vec;
use nsh_crypto::{key_digest, ClientKem, Secret, Transcript};
use nsh_wire::hello::RANDOM_LEN;
use nsh_wire::{len_field, ClientHello, Expect, Kind, CLIENT_HELLO_LEN};

impl<A: Attester, S: UserSigner> Client<A, S> {
    /// `kem_seed` and `random` must be fresh random bytes, made for this
    /// one connection.
    pub fn new(
        cfg: Config,
        attester: A,
        signer: S,
        kem_seed: Secret<32>,
        random: [u8; RANDOM_LEN],
    ) -> Result<Self, Refusal> {
        super::grant::check(&cfg.grant, &signer)?;
        let subject = key_digest(signer.ed_public(), signer.ml_public());
        let (kem, ek) = ClientKem::new(kem_seed);
        let mut body = [0u8; CLIENT_HELLO_LEN];
        let n = ClientHello { random: &random, kem_ek: &ek }.encode(&mut body);
        let n = n.map_err(Refusal::Wire)?;
        let mut out = Vec::from(len_field(n, Expect::ClientHello).map_err(Refusal::Wire)?);
        out.push(Kind::ClientHello.byte());
        out.extend_from_slice(&body[..n]);
        let mut th = Transcript::default();
        th.push(Kind::ClientHello, &body[..n]);
        Ok(Client {
            cfg,
            attester,
            signer,
            stage: Stage::ServerHello,
            th,
            th_hello: [0; 64],
            bound: Bound { subject: Some(subject), ..Bound::default() },
            doc_hash: None,
            kem: Some(kem),
            hs: None,
            sealer: None,
            opener: None,
            framer: Framer::default(),
            out,
            events: Vec::new(),
            record: Vec::new(),
        })
    }
}
