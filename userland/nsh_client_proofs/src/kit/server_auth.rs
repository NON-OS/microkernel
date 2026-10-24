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

//! The server's half of the login: `USER_AUTH` is checked for what the
//! client is responsible for (the grant parses, it names these keys, the
//! role authority signed it, and the user signed the transcript), then
//! `AUTH_OK` is sealed and both directions move to the session keys.

use super::keys;
use nonos_ed25519::Signature;
use nsh_crypto::{key_digest, labelled, session_keys, Keys, Label, Opener, Sealer};
use nsh_wire::{AuthOk, Kind, UserAuth};

pub const SESSION_ID: [u8; 16] = [0x5E; 16];

impl super::server::Server {
    /// Whether a `USER_AUTH` body passes; it joins the transcript if so.
    pub fn user_auth(&mut self, body: &[u8]) -> bool {
        let Ok(ua) = UserAuth::parse(body) else { return false };
        let both = |ed: &[u8; 32], ml: &[u8; 1952], msg: &[u8], sig: &nsh_wire::HybridSig| {
            nonos_ed25519::verify(ed, msg, &Signature::from_bytes(sig.ed))
                && nonos_mldsa65::verify(ml, msg, sig.ml)
        };
        let role = keys::role();
        let issuer = both(
            &role.ed.public,
            &role.ml_public,
            &labelled(Label::Grant, ua.grant.signed),
            &ua.grant.sig,
        );
        let subject = ua.grant.subject == &key_digest(ua.user.ed, ua.user.ml);
        let th = self.th.without_sigs(Kind::UserAuth, ua.signed);
        let user = both(ua.user.ed, ua.user.ml, &labelled(Label::UserSig, &th), &ua.sig);
        if issuer && subject && user {
            self.th.push(Kind::UserAuth, body);
        }
        issuer && subject && user
    }

    pub fn auth_ok_body() -> Vec<u8> {
        let mut body = vec![0u8; 64];
        let n = AuthOk::encode(&SESSION_ID, 600, &[b"CoreExec", b"IPC"], &mut body).unwrap();
        body.truncate(n);
        body
    }

    /// Seal `AUTH_OK` and move to the session keys.
    pub fn auth_ok(&mut self, body: &[u8]) -> Vec<u8> {
        let wire = self.seal(Kind::AuthOk, body);
        self.th.push(Kind::AuthOk, body);
        let Keys { c2s, s2c } = session_keys(self.hs.take().unwrap(), &self.th.digest());
        (self.sealer, self.opener) = (Some(Sealer::new(s2c)), Some(Opener::new(c2s)));
        wire
    }
}
