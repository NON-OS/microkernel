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

//! The model's records: the function's one claims entry, its wipe record as
//! table.rs keeps it, and the ground truth beside them.

use super::end_rule::End;
use super::holder_rule::Name;
use super::marker_rule::Marker;
use super::method_rule::Method;
use super::model_stage::types::Stage;
use super::phase_rule::Phase;

/// types.rs `Claim` for the one function, no grant path inside it: the
/// stage model proves those.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Entry {
    pub pid: usize,
    pub epoch: u64,
    pub stage: Stage,
    pub leavers: u32,
}

/// table.rs `Saved`, cut to the registers the model's function holds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Saved {
    pub command: u16,
    pub devctl: Option<u16>,
}

/// table.rs `Wipe`; its caps and address are the one function's own.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Wipe {
    pub epoch: u64,
    pub by: Option<Name>,
    pub phase: Phase,
    pub method: Option<Method>,
    pub marker: Option<Marker>,
    pub saved: Option<Saved>,
    pub bme_off: bool,
    pub busy: bool,
}

/// What happened, as the invariants judge it, apart from what the kernel
/// keeps: when the wipe was owed and when it last began to be pending, the
/// last loss, the reset writes since the last check, how the last wipe
/// ended, and whether a lost one was asked again since the last admit.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Truth {
    pub owed_at: u64,
    pub since: u64,
    pub lost_at: Option<u64>,
    pub writes: u8,
    pub last_end: Option<End>,
    pub rearmed: bool,
}
