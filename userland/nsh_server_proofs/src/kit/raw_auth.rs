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

//! The raw client's login: a USER_AUTH body signed over the transcript, and
//! the session keys once AUTH_OK is in.

use super::raw::Raw;
use super::roles_build::grant::GrantB;
use super::roles_build::keys::{key, K};
use nsh_crypto::{labelled, session_keys, Keys, Label, Opener, Sealer};
use nsh_wire::Kind;

impl Raw {
    /// A `USER_AUTH` body for `grant`, signed by `user` over the transcript.
    pub fn user_auth(&self, grant: &GrantB, user: K) -> Vec<u8> {
        let k = key(user);
        let g = grant.bytes();
        let mut body = k.public();
        body.extend_from_slice(&u16::try_from(g.len()).expect("grant").to_le_bytes());
        body.extend_from_slice(&g);
        let digest = labelled(Label::UserSig, &self.th.without_sigs(Kind::UserAuth, &body));
        body.extend_from_slice(&k.sign(&digest));
        body
    }

    /// After `AUTH_OK`: the session keys, from the transcript through it.
    pub fn logged_in(&mut self, user_auth: &[u8], auth_ok: &[u8]) {
        self.th.push(Kind::UserAuth, user_auth);
        self.th.push(Kind::AuthOk, auth_ok);
        let Keys { c2s, s2c } = session_keys(self.hs.take().expect("hs"), &self.th.digest());
        (self.sealer, self.opener) = (Some(Sealer::new(c2s)), Some(Opener::new(s2c)));
    }
}
