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
//! The machine's half: a NONOSATT v5 document, read and its quote checked
//! by nsh_attest, that answers this receipt's challenge, from a trusted
//! release, with the IOMMU enforcing, the registry complete and the pinned AK.

use nsh_attest::{steps, Doc};

use super::Refusal;
use crate::napr::doc_form::cleared;
use crate::napr::{challenge, Receipt};
use crate::trust::{Release, Trust};

pub fn doc<'a, 't>(r: &Receipt<'a>, trust: &'t Trust) -> Result<(Doc<'a>, &'t Release), Refusal> {
    let doc = Doc::parse(r.doc).map_err(Refusal::Doc)?;
    if *doc.challenge != challenge(r.bound) {
        return Err(Refusal::Challenge);
    }
    if !cleared(&doc) {
        return Err(Refusal::Unsigned);
    }
    steps::quote(&doc).map_err(Refusal::Doc)?;
    if *doc.ak_public != trust.ak {
        return Err(Refusal::Ak);
    }
    let release = release(&doc, trust).ok_or(Refusal::Release)?;
    if !doc.iommu_enforcing || doc.unconfined_grants != 0 {
        return Err(Refusal::Dma);
    }
    if !doc.registry_complete {
        return Err(Refusal::Registry);
    }
    Ok((doc, release))
}

/// nsh_attest's release rule (steps/release.rs), over the verifier's trust.
fn release<'t>(doc: &Doc<'_>, trust: &'t Trust) -> Option<&'t Release> {
    let pcr = |i: u8| doc.pcr(i).filter(|v| v.iter().any(|b| *b != 0));
    let pcr9 = pcr(9)?;
    let r = trust.releases.iter().find(|r| r.pcr9 == *pcr9)?;
    (*pcr(8)? == r.pcr8 && *pcr(14)? == r.pcr14).then_some(r)
}
