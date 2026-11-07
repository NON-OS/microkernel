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

//! The user and the client: the user's keys from nsh_roles_proofs, a base
//! grant from its builder, the client's fleet policy naming the spec's
//! enrolment authority, and the real `NONOSATT` checker over the spec's
//! releases.

use super::attest_build::keys::{hybrid, AUTHORITY};
use super::host::{built, HOST_NAME};
use super::roles_build::grant::GrantB;
use super::roles_build::keys::{key, K};
use super::roles_build::NOW;
use nsh_client::{Config, HybridKey, NonosAttester, Policy, Rand, UserSig, UserSigner};
use nsh_wire::keys::{ED_PUBLIC_LEN, ML_PUBLIC_LEN};

/// A user key of nsh_roles_proofs'.
pub struct User(pub K);

impl UserSigner for User {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN] {
        &key(self.0).ed.public
    }
    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN] {
        &key(self.0).ml_pk
    }
    fn sign(&self, digest: &[u8; 64], _: Rand<'_>) -> Option<UserSig> {
        let sig = key(self.0).sign(digest);
        let (ed, ml) = sig.split_at(64);
        Some(UserSig { ed: ed.try_into().ok()?, ml: Box::new(ml.try_into().ok()?) })
    }
}

pub fn policy() -> Policy {
    let a = hybrid(AUTHORITY);
    let authority = HybridKey { ed: a.ed.public, ml: a.ml_public.clone() };
    Policy { fleet_id: built().fleet, authorities: vec![authority], revoked: Vec::new() }
}

pub fn attester() -> NonosAttester {
    NonosAttester { policy: built().policy() }
}

pub fn client_config(grant: &GrantB) -> Config {
    Config { host: HOST_NAME.to_vec(), now_ms: NOW, policy: policy(), grant: grant.bytes() }
}
