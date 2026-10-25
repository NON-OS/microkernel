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

//! Step 5, the NONOS chain: PCR 9 names the admitted kernel and so picks
//! the one accepted release; PCR 8 (the loader's own measurement) and PCR
//! 14 (the capsules and proofs) must then be that release's. A zero value
//! is what a boot that extended nothing leaves, and is refused.

use crate::{Doc, Policy, Refusal, Release};

pub fn release<'p>(doc: &Doc<'_>, policy: &'p Policy) -> Result<&'p Release, Refusal> {
    let pcr = |i: u8, r: Refusal| doc.pcr(i).filter(|v| v.iter().any(|b| *b != 0)).ok_or(r);
    let pcr9 = pcr(9, Refusal::Pcr9)?;
    let release = policy.release(pcr9).ok_or(Refusal::Pcr9)?;
    if *pcr(8, Refusal::Pcr8)? != release.pcr8 {
        return Err(Refusal::Pcr8);
    }
    if *pcr(14, Refusal::Pcr14)? != release.pcr14 {
        return Err(Refusal::Pcr14);
    }
    Ok(release)
}
