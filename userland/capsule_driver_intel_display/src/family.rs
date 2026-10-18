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

//! The Intel graphics families this driver knows, by their tag in the
//! device id data files, and the display generation each one carries.

use crate::engine::gen::{Gen, GEN11, GEN12, GEN12_3, GEN7_5, GEN8, GEN9};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Haswell,
    Broadwell,
    Skylake,
    KabyLake,
    CoffeeLake,
    CometLake,
    IceLake,
    TigerLake,
    RocketLake,
    AlderLakeS,
    AlderLakeP, // with Raptor Lake P, display 13, taken over as Gen12
    RaptorLakeS,
    RaptorLakeP,
}

impl Family {
    /// The family a data file's three-letter tag names.
    pub const fn from_tag(t: [u8; 3]) -> Option<Family> {
        Some(match &t {
            b"hsw" => Family::Haswell,
            b"bdw" => Family::Broadwell,
            b"skl" => Family::Skylake,
            b"kbl" => Family::KabyLake,
            b"cfl" => Family::CoffeeLake,
            b"cml" => Family::CometLake,
            b"icl" => Family::IceLake,
            b"tgl" => Family::TigerLake,
            b"rkl" => Family::RocketLake,
            b"ads" => Family::AlderLakeS,
            b"adp" => Family::AlderLakeP,
            b"rps" => Family::RaptorLakeS,
            b"rpp" => Family::RaptorLakeP,
            _ => return None,
        })
    }

    pub fn gen(self) -> &'static Gen {
        match self {
            Family::Haswell => &GEN7_5,
            Family::Broadwell => &GEN8,
            Family::Skylake | Family::KabyLake | Family::CoffeeLake | Family::CometLake => &GEN9,
            Family::IceLake => &GEN11,
            Family::RocketLake => &GEN12_3,
            _ => &GEN12,
        }
    }

    /// Gen9 alone sets modes; the rest keep firmware's until their unit.
    pub fn sets_modes(self) -> bool {
        *self.gen() == GEN9
    }
}
