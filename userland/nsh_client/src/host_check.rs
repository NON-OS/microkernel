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

//! Accepting a host: the attestation document for this handshake's
//! challenge, then the enrolment against what the document says, then the
//! firmware PCRs against the enrolment. The same check serves the live
//! client and the offline record checker, which has no registry entries,
//! no typed host name and no clock.

use crate::ct::eq;
use crate::enrol_check::{self, Expected, Online};
use crate::{Attester, HostRefusal, Policy};
use nsh_crypto::{attest_challenge, key_digest};
use nsh_wire::{Enrol, HybridPublic};

pub(crate) struct Claim<'a> {
    pub host: HybridPublic<'a>,
    pub enrol: &'a Enrol<'a>,
    pub doc: &'a [u8],
    pub entries: Option<&'a [u8]>,
    pub th_server_hello: &'a [u8; 64],
}

pub(crate) fn check<A: Attester + ?Sized>(
    c: &Claim<'_>,
    policy: &Policy,
    online: Option<Online<'_>>,
    attester: &A,
) -> Result<(), HostRefusal> {
    let challenge = attest_challenge(c.th_server_hello, c.host.ed, c.host.ml);
    let quoted = attester.check(c.doc, &challenge, c.entries).map_err(HostRefusal::Attest)?;
    let in_doc =
        quoted.challenge_at.checked_add(32).and_then(|end| c.doc.get(quoted.challenge_at..end));
    if !in_doc.is_some_and(|d| eq(d, &challenge)) {
        return Err(HostRefusal::Challenge);
    }
    let host_keys = key_digest(c.host.ed, c.host.ml);
    let expected = Expected { policy, ak: &quoted.ak_public, host_keys: &host_keys, online };
    enrol_check::check(c.enrol, &expected).map_err(HostRefusal::Enrol)?;
    let pcrs = quoted.firmware_pcrs.iter().zip(c.enrol.pcrs.iter());
    match pcrs.fold(true, |ok, (q, e)| eq(q, *e) & ok) {
        true => Ok(()),
        false => Err(HostRefusal::FirmwarePcrs),
    }
}
