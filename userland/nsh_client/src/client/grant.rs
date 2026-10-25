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

//! The grant the client is given is checked before anything is sent: it
//! must parse as the server will read it, be a base grant, and name the
//! user key the client signs with. A grant the server would refuse as a
//! broken `USER_AUTH` is never sent.

use crate::{Refusal, UserSigner};
use nsh_crypto::key_digest;
use nsh_wire::{Grant, GrantKind, WireError, MAX_GRANT};

pub(super) fn check<S: UserSigner>(grant: &[u8], signer: &S) -> Result<(), Refusal> {
    let size = (1..=MAX_GRANT).contains(&grant.len());
    let g = Grant::parse(grant).map_err(Refusal::Unsendable)?;
    if !size || g.kind != GrantKind::Base {
        return Err(Refusal::Unsendable(WireError::Value));
    }
    match g.subject == &key_digest(signer.ed_public(), signer.ml_public()) {
        true => Ok(()),
        false => Err(Refusal::GrantSubject),
    }
}
