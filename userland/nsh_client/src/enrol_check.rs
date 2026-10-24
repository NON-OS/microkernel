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

//! The enrolment certificate (nsh-v1-attest.md step 4): signed, both
//! halves, by the fleet's enrolment authority, for this fleet, AK and host
//! keys, and not withdrawn; online, also for the typed host and unexpired.

use crate::ct::eq;
use crate::{hybrid, EnrolRefusal, Policy};
use nonos_hash::Sha512;
use nsh_crypto::{labelled, Label};
use nsh_wire::Enrol;

/// What only a live connection knows: the host the user typed and now.
#[derive(Clone, Copy)]
pub(crate) struct Online<'a> {
    pub host: &'a [u8],
    pub now_ms: u64,
}

pub(crate) struct Expected<'a> {
    pub policy: &'a Policy,
    pub ak: &'a [u8; 64],
    pub host_keys: &'a [u8; 64],
    pub online: Option<Online<'a>>,
}

pub(crate) fn check(e: &Enrol<'_>, x: &Expected<'_>) -> Result<(), EnrolRefusal> {
    let auth = x.policy.authority(e.authority).ok_or(EnrolRefusal::Authority)?;
    let digest = labelled(Label::Enrol, e.signed);
    hybrid::verify(&auth.ed, &auth.ml, &digest, &e.sig).map_err(EnrolRefusal::Sig)?;
    if !eq(e.fleet_id, &x.policy.fleet_id) {
        return Err(EnrolRefusal::Fleet);
    }
    if !eq(e.ak_public, x.ak) {
        return Err(EnrolRefusal::Ak);
    }
    if !eq(e.host_keys, x.host_keys) {
        return Err(EnrolRefusal::HostKeys);
    }
    let digest = certificate_digest(e);
    if x.policy.revoked.iter().any(|r| eq(r, &digest)) {
        return Err(EnrolRefusal::Revoked);
    }
    let Some(online) = x.online else { return Ok(()) };
    if !eq(e.host, online.host) {
        return Err(EnrolRefusal::HostName);
    }
    match online.now_ms >= e.not_after_ms {
        true => Err(EnrolRefusal::Expired),
        false => Ok(()),
    }
}

/// SHA-512 of the whole certificate: the signed bytes and both signatures.
fn certificate_digest(e: &Enrol<'_>) -> [u8; 64] {
    let mut h = Sha512::new();
    h.update(e.signed);
    h.update(e.sig.ed);
    h.update(e.sig.ml);
    h.finalize()
}
