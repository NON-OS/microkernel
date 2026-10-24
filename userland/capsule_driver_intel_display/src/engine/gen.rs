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

//! What differs between Intel display generations for a takeover: how
//! many pipes there are, whether the eDP transcoder exists, and which
//! primary plane layout the pipes carry, and the GGTT's entry format.
//! Mode setting stays Gen9's alone.

pub use super::ggtt_pte::Pte;
use PlaneKind::{Primary, Universal, Universal11};

/// The register layout of each pipe's primary plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneKind {
    /// Haswell and Broadwell: DSPCNTR, DSPSTRIDE in bytes, no size
    /// register; the plane is as large as PIPESRC (BDW PRM Vol 2c).
    Primary,
    /// Gen9: universal plane 1 (SKL PRM Vol 2c, PLANE_CTL).
    Universal,
    /// Gen11 and later: universal plane 1 with a five-bit Source Pixel
    /// Format at 27:23 and a wider stride (ICL PRM Vol 2c, PLANE_CTL,
    /// PLANE_STRIDE; field widths unverified against the volumes).
    Universal11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gen {
    /// Pipes A onward, each with the transcoder of its letter.
    pub pipes: u32,
    /// The eDP transcoder at 0x6F000, Haswell to Gen11. Gen12 drives eDP
    /// from a pipe's own transcoder (TGL PRM Vol 12; unverified).
    pub edp_transcoder: bool,
    pub plane: PlaneKind,
    pub ggtt: Pte,
}

const fn gen(pipes: u32, edp_transcoder: bool, plane: PlaneKind, ggtt: Pte) -> Gen {
    Gen { pipes, edp_transcoder, plane, ggtt }
}

/// Haswell: Broadwell's plane, the GGTT's narrow entries.
pub const GEN7_5: Gen = gen(3, true, Primary, Pte::Narrow);
pub const GEN8: Gen = gen(3, true, Primary, Pte::Wide);
pub const GEN9: Gen = gen(3, true, Universal, Pte::Wide);
pub const GEN11: Gen = gen(3, true, Universal11, Pte::Wide);
/// Rocket Lake: Gen12 with three pipes (RKL PRM Vol 12; unverified).
pub const GEN12_3: Gen = gen(3, false, Universal11, Pte::Wide);
/// Tiger Lake, Alder Lake and Raptor Lake: four pipes, A to D.
pub const GEN12: Gen = gen(4, false, Universal11, Pte::Wide);
