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

//! The money-grade DEEP STARK prover, streamed. The trace lives as coefficients
//! and every pass extends one coset at a time, so proving needs the memory of a
//! coset rather than of the evaluation domain. Byte-identical to the
//! materialized prover it replaced: same transcript order, same field values,
//! same trees.

mod commit;
mod compose;
mod coset;
mod deep;
mod entry;
mod frame;
mod ood;
mod queries;
mod run;
mod setup;

pub(in crate::air) use crate::poly::batch_inv;
pub(in crate::air) use commit::wide_streamed;
pub(in crate::air) use compose::{over_domain, BLOCK};
pub(in crate::air) use coset::{extend, periodic_coeffs, trace_coeffs};
pub(in crate::air) use deep::over_domain as deep_over_domain;
pub use entry::{stark_prove_ext, stark_prove_ext_blown, stark_prove_ext_blown_bound};
pub(in crate::air) use frame::{comp_at_z, ood_frame, periodic_at_z};
pub(in crate::air) use ood::draw_ood_point_ext;
pub(in crate::air) use queries::eval_base;
pub(in crate::air) use setup::Domain;
