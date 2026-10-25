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

//! A valid `USER_AUTH` body: the user's keys, the base grant, and the
//! user's signature over `H("nsh/1 user-sig\0" || TH)`.

use super::keys::{key, K};
use super::{put_u16, Spoil, TH};
use nsh_crypto::{labelled, Label};

#[derive(Debug, Clone)]
pub struct UserAuthB {
    pub user: K,
    pub grant: Vec<u8>,
    pub th: [u8; 64],
    pub signer: K,
    pub spoil: Spoil,
}

impl UserAuthB {
    pub fn new(grant: Vec<u8>) -> Self {
        UserAuthB { user: K::User, grant, th: TH, signer: K::User, spoil: Spoil::None }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut out = key(self.user).public();
        put_u16(&mut out, u16::try_from(self.grant.len()).expect("grant too long"));
        out.extend_from_slice(&self.grant);
        let mut sig = key(self.signer).sign(&labelled(Label::UserSig, &self.th));
        self.spoil.apply(&mut sig);
        out.extend_from_slice(&sig);
        out
    }
}
