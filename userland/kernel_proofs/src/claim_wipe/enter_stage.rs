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

//! The claims' stage rule and types, the kernel's own files mounted by path
//! for the enter model (L78): how an entry standing answers a claimant, what
//! a release and an exit move it to, and when it goes.

// Mounted once, in claim_stage.
use crate::proofs_7::claim_stage::{stage_rule, types};

pub(super) use stage_rule::{exited, gone, over, retired};
pub(super) use types::{ClaimError, Stage};
