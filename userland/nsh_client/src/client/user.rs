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

//! `USER_AUTH`: the user's keys, the base grant, and the user's signature
//! over the transcript, which now holds the accepted `HOST_PROOF`. It is
//! read back with the server's own parser before it is sealed, so the
//! client never sends a body the server would refuse as broken.

use super::stage::Stage;
use super::Client;
use crate::{Attester, Rand, Refusal, UserSigner};
use alloc::vec::Vec;
use nsh_crypto::{labelled, Label};
use nsh_wire::{Expect, Kind, UserAuth, WireError};

impl<A: Attester, S: UserSigner> Client<A, S> {
    pub(super) fn user_auth(&mut self, rand: Rand<'_>) -> Result<(), Refusal> {
        let grant = &self.cfg.grant;
        let n = u16::try_from(grant.len()).map_err(|_| Refusal::Unsendable(WireError::Oversize))?;
        let mut body = Vec::with_capacity(2048 + 2 + grant.len() + 3373);
        body.extend_from_slice(self.signer.ed_public());
        body.extend_from_slice(self.signer.ml_public());
        body.extend_from_slice(&n.to_le_bytes());
        body.extend_from_slice(grant);
        let digest = labelled(Label::UserSig, &self.th.without_sigs(Kind::UserAuth, &body));
        let sig = self.signer.sign(&digest, rand).ok_or(Refusal::Sign)?;
        body.extend_from_slice(&sig.ed);
        body.extend_from_slice(&sig.ml[..]);
        UserAuth::parse(&body).map_err(Refusal::Unsendable)?;
        self.send(Kind::UserAuth, &body, Expect::UserAuth)?;
        self.th.push(Kind::UserAuth, &body);
        self.stage = Stage::AuthResult;
        Ok(())
    }
}
