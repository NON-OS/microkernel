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

//! Intel's display microcontroller firmware (DMC), under the owner's
//! Decision 19(c): a blob is loaded only when its SHA-256 is sealed in
//! `data/dmc_sealed.txt` under the blob name its family needs. The file
//! ships empty, so no DMC is loaded, and the engine keeps the display's
//! low-power states (DC5, DC6) off: the cost is idle power with the panel
//! lit, never a picture.

use crate::dmc_parse::{parse, Sealed};
use crate::family::Family;

static SEALED: Sealed = parse(include_bytes!("../data/dmc_sealed.txt"));

/// The DMC blob a family runs, by the stem Intel publishes it under,
/// without a version: the owner chooses the version when sealing. Haswell
/// and Broadwell have none. Which later parts share a blob is unverified.
pub fn blob(f: Family) -> Option<&'static str> {
    Some(match f {
        Family::Haswell | Family::Broadwell => return None,
        Family::Skylake => "skl_dmc",
        Family::KabyLake | Family::CoffeeLake | Family::CometLake => "kbl_dmc",
        Family::IceLake => "icl_dmc",
        Family::TigerLake => "tgl_dmc",
        Family::RocketLake => "rkl_dmc",
        Family::AlderLakeS | Family::RaptorLakeS => "adls_dmc",
        Family::AlderLakeP | Family::RaptorLakeP => "adlp_dmc",
    })
}

/// `bytes`, when they are a blob sealed for `f` in `seal`.
pub fn admit_in<'a>(seal: &Sealed, f: Family, bytes: &'a [u8]) -> Option<&'a [u8]> {
    let name = blob(f)?.as_bytes();
    let digest = nonos_hash::sha256(bytes);
    seal.entries[..seal.len]
        .iter()
        .any(|e| &e.name[..e.name_len] == name && e.digest == digest)
        .then_some(bytes)
}

/// `bytes`, when they are a blob the owner sealed for `f`.
pub fn admit(f: Family, bytes: &[u8]) -> Option<&[u8]> {
    admit_in(&SEALED, f, bytes)
}
