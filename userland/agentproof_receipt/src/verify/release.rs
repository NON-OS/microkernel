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

//! The release verdict, given apart from verification as nsh_attest gives
//! it: a verified receipt is from a release only if PCR 14, replayed from
//! its log, records at least one capsule and every one admitted on its STARK
//! proof under the vendor's root (nsh_attest steps::capsules). A development
//! image admits each by its path alone, so its receipts verify and are not
//! from a release; which one they are is said, and `--release` refuses them.

use alloc::vec::Vec;

use nsh_attest::log::Admitted;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Release {
    /// Every capsule admitted under the vendor's root.
    Release,
    /// Each kind PCR 14 records that a release does not admit, with how many;
    /// empty when it records no capsule at all.
    NotRelease(Vec<(Admitted, usize)>),
}

pub fn release(capsules: &[Admitted]) -> Release {
    let kinds = [Admitted::PathOnly, Admitted::Developer, Admitted::LocalBuild];
    let count = |k: Admitted| capsules.iter().filter(|c| **c == k).count();
    let not: Vec<_> = kinds.into_iter().map(|k| (k, count(k))).filter(|(_, n)| *n > 0).collect();
    match (capsules.is_empty(), not.is_empty()) {
        (false, true) => Release::Release,
        _ => Release::NotRelease(not),
    }
}

impl Release {
    /// "release" or "not a release: 28 path-only capsules".
    pub fn said(&self) -> alloc::string::String {
        let Self::NotRelease(not) = self else { return "release".into() };
        let each: Vec<_> =
            not.iter().map(|(k, n)| alloc::format!("{n} {} capsules", k.name())).collect();
        match each.is_empty() {
            true => "not a release: PCR 14 records no capsule".into(),
            false => alloc::format!("not a release: {}", each.join(", ")),
        }
    }
}
