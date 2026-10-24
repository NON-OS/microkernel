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

//! Host proofs for zero-state. The kernel's keep rule and the disk plan
//! parser it reads are mounted from their source, so what is proved here is
//! what the kernel runs.

// The rule the store write asks: what a write may change on this boot.
// The kernel keeps its items crate-wide; only the tests call them here.
#[allow(dead_code)]
#[path = "../../../src/security/zerostate/keep_rule.rs"]
pub mod keep_rule;

// The disk plan: how an installed disk is told from a live one.
#[path = "../../../src/fs/blockfs_volume/plan.rs"]
pub mod plan;
#[path = "../../../src/fs/blockfs_volume/plan_types.rs"]
pub mod plan_types;

pub mod model;
pub mod sweep;

#[cfg(test)]
mod chain_tests;
#[cfg(test)]
mod rule_tests;
