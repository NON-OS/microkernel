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

//! Step 4: the enrolment certificate. The fleet's authority signed it with
//! both halves of its hybrid key; it names this document's AK, these host
//! keys, the host the user asked for and this fleet; it has not run out;
//! and the policy has not revoked it.

use nonos_ed25519::Signature;
use nonos_hash::sha512;
use nsh_crypto::{key_digest, labelled, Label};
use nsh_wire::Enrol;

use crate::{Doc, Evidence, Policy, Refusal, Session};

pub fn enrol<'a>(
    ev: &Evidence<'a>,
    doc: &Doc<'_>,
    s: &Session<'_>,
    policy: &Policy,
) -> Result<Enrol<'a>, Refusal> {
    let e = Enrol::parse(ev.enrol).map_err(|_| Refusal::EnrolFormat)?;
    let a = policy.authority(e.authority).ok_or(Refusal::EnrolAuthority)?;
    let digest = labelled(Label::Enrol, e.signed);
    let ed = nonos_ed25519::verify(&a.ed, &digest, &Signature::from_bytes(e.sig.ed));
    let ml = nonos_mldsa65::verify(&a.ml, &digest, e.sig.ml);
    if !(ed && ml) {
        return Err(Refusal::EnrolSignature);
    }
    let checks = [
        (e.fleet_id == policy.fleet_id(), Refusal::EnrolFleet),
        (e.host == s.host, Refusal::EnrolHost),
        (e.ak_public == doc.ak_public, Refusal::EnrolAk),
        (*e.host_keys == key_digest(ev.host_ed, ev.host_ml), Refusal::EnrolHostKeys),
        (s.now_ms < e.not_after_ms, Refusal::EnrolExpired),
        (!policy.revoked(&sha512(ev.enrol)), Refusal::EnrolRevoked),
    ];
    match checks.into_iter().find(|c| !c.0) {
        Some((_, refusal)) => Err(refusal),
        None => Ok(e),
    }
}
