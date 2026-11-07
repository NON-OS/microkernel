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

//! The `NONOSATT` version 5 checker, `nsh_attest`, as the client's
//! `Attester`: steps 1, 2, 3, 5 (the release, the log, PCR 14), 6 and 7 of
//! docs/nsh/nsh-v1-attest.md, in that order, against the fleet's releases.
//! Step 4, the enrolment, and the firmware half of step 5 are the client's
//! own. A record checked offline has no registry entries; its registry
//! root is bound through the quote, which step 3 checks.

use crate::nonos_refusal::refusal;
use crate::{AttestRefusal, Attester, Quoted};
use nsh_attest::steps::{capsules, dma, log, quote, registry, release};
use nsh_attest::{Doc, Policy};
use nsh_wire::Enrol;

/// The fleet's accepted releases, from its `nsh_attest::Policy`.
pub struct NonosAttester {
    pub policy: Policy,
}

impl Attester for NonosAttester {
    fn check(
        &self,
        doc: &[u8],
        challenge: &[u8; 32],
        entries: Option<&[u8]>,
        enrol: &Enrol<'_>,
    ) -> Result<Quoted, AttestRefusal> {
        let d = Doc::parse(doc).map_err(refusal)?;
        if d.challenge != challenge {
            return Err(AttestRefusal::Challenge);
        }
        quote(&d).map_err(refusal)?;
        let rel = release(&d, &self.policy).map_err(refusal)?;
        capsules(&log(&d, rel, enrol).map_err(refusal)?).map_err(refusal)?;
        dma(&d).map_err(refusal)?;
        if let Some(entries) = entries {
            registry(&d, entries, rel).map_err(refusal)?;
        }
        let at = (d.challenge.as_ptr() as usize).checked_sub(doc.as_ptr() as usize);
        let challenge_at = at.filter(|a| a + 32 <= doc.len()).ok_or(AttestRefusal::Parse)?;
        let pcr = |i| d.pcr(i).copied().ok_or(AttestRefusal::Parse);
        Ok(Quoted {
            challenge_at,
            ak_public: *d.ak_public,
            firmware_pcrs: [pcr(0)?, pcr(1)?, pcr(2)?, pcr(7)?],
        })
    }
}
