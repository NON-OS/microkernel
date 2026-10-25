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

//! `HOST_PROOF`, and the check that must pass before any credential
//! leaves: the host keys' hybrid signature over the transcript, then the
//! attestation document for this handshake's challenge, the enrolment and
//! the firmware PCRs. Only then is the frame taken into the transcript and
//! `USER_AUTH` made. Any refusal is answered `ABORT(POLICY)`.

use super::Client;
use crate::enrol_check::Online;
use crate::host_check::{self, Claim};
use crate::{hybrid, Attester, Event, HostRefusal, Rand, Refusal, UserSigner};
use nsh_crypto::{labelled, Label};
use nsh_wire::{abort, HostProof, Kind, WireError};

impl<A: Attester, S: UserSigner> Client<A, S> {
    pub(super) fn host_proof(
        &mut self,
        kind: Kind,
        body: &[u8],
        rand: Rand<'_>,
    ) -> Result<(), Refusal> {
        match kind {
            Kind::HostProof => {}
            Kind::Abort => return Err(Refusal::Aborted(abort(body).map_err(Refusal::Wire)?)),
            _ => return Err(Refusal::Wire(WireError::Order)),
        }
        let hp = HostProof::parse(body).map_err(Refusal::Wire)?;
        let signed = labelled(Label::HostSig, &self.th.without_sigs(Kind::HostProof, hp.signed));
        hybrid::verify(hp.host.ed, hp.host.ml, &signed, &hp.sig)
            .map_err(|h| Refusal::Host(HostRefusal::HostSig(h)))?;
        let claim = Claim {
            host: hp.host,
            enrol: &hp.enrol,
            doc: hp.doc,
            entries: Some(hp.entries),
            th_server_hello: &self.th_hello,
        };
        let online = Online { host: &self.cfg.host, now_ms: self.cfg.now_ms };
        host_check::check(&claim, &self.cfg.policy, Some(online), &self.attester)
            .map_err(Refusal::Host)?;
        self.doc_hash = Some(nonos_hash::sha512(hp.doc));
        self.th.push(Kind::HostProof, body);
        self.user_auth(rand)?;
        self.events.push(Event::HostAccepted);
        Ok(())
    }
}
