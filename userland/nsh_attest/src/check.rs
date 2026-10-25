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

//! The whole check, in the order nsh-v1-attest.md gives. The first refusal
//! ends it; nothing is skipped and nothing is left to the user.

use crate::steps::{capsules, challenge, dma, enrol, firmware, log, quote, registry, release};
use crate::{Doc, Evidence, Policy, Refusal, Release, Session};

/// What a host that passed has proved, for the client's log line
/// `attest: ok release=<version> pcr9=<hex>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict<'p> {
    pub release: &'p Release,
    pub pcr9: [u8; 32],
    pub registry_root: [u8; 32],
    pub ak_public: [u8; 64],
}

pub fn check<'p>(
    ev: &Evidence<'_>,
    s: &Session<'_>,
    policy: &'p Policy,
) -> Result<Verdict<'p>, Refusal> {
    let doc = Doc::parse(ev.doc)?;
    challenge(&doc, ev, s)?;
    quote(&doc)?;
    let cert = enrol(ev, &doc, s, policy)?;
    let rel = release(&doc, policy)?;
    capsules(&log(&doc, rel, &cert)?)?;
    firmware(&doc, &cert)?;
    dma(&doc)?;
    registry(&doc, ev.entries, rel)?;
    Ok(Verdict {
        release: rel,
        pcr9: rel.pcr9,
        registry_root: *doc.registry_root,
        ak_public: *doc.ak_public,
    })
}
