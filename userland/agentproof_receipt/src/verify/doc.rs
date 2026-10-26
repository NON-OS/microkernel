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

//! The doc's half of a receipt: one NONOSATT v5 document, read and checked
//! by nsh_attest, that answers this receipt's challenge under the pinned AK;
//! from a trusted release, by nsh_attest's rule (PCR 9 names it and PCR 8 is
//! its own); with the IOMMU enforcing; whose entries fold to the registry
//! root it bound; and whose capsules replay to its PCR 14 (capsules.rs).
//! What admitted those capsules is the release verdict's (release.rs).

use alloc::vec::Vec;

use nsh_attest::log::Admitted;
use nsh_attest::{steps, Doc};

use super::Refusal;
use crate::napr::doc_form::cleared;
use crate::napr::{challenge, Receipt};
use crate::trust::{Release, Trust};

pub struct Checked<'a, 't> {
    pub doc: Doc<'a>,
    pub release: &'t Release,
    pub capsules: Vec<Admitted>,
}

pub fn doc<'a, 't>(r: &Receipt<'a>, trust: &'t Trust) -> Result<Checked<'a, 't>, Refusal> {
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
    // nsh_attest's own rule: an IOMMU present and enforcing, no grant
    // around it, and no device left in a domain that reaches all of memory.
    steps::dma(&doc).map_err(|_| Refusal::Dma)?;
    // The entries are what ran only if they fold to the root the quote bound.
    if steps::fold(r.entries) != *doc.registry_root {
        return Err(Refusal::Registry);
    }
    let capsules = super::capsules::replay(r.capsules, doc.pcr(14).ok_or(Refusal::Capsules)?)?;
    Ok(Checked { doc, release, capsules })
}

/// nsh_attest's release rule (steps/release.rs), over the verifier's trust.
fn release<'t>(doc: &Doc<'_>, trust: &'t Trust) -> Option<&'t Release> {
    let pcr = |i: u8| doc.pcr(i).filter(|v| v.iter().any(|b| *b != 0));
    let pcr9 = pcr(9)?;
    let r = trust.releases.iter().find(|r| r.pcr9 == *pcr9)?;
    (*pcr(8)? == r.pcr8).then_some(r)
}
