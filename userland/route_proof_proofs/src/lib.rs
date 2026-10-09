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

//! The anonymity route proofs, run on the host against the real source.

#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod attest_protocol;
#[path = "../../route_proof/src/authorize.rs"]
pub mod authorize;
#[path = "../../route_proof/src/board.rs"]
pub mod board;
#[path = "../../route_proof/src/facts.rs"]
pub mod facts;
#[path = "../../route_proof/src/frame.rs"]
pub mod frame;
#[path = "../../route_proof/src/report.rs"]
pub mod report;
#[path = "../../route_proof/src/verdict.rs"]
pub mod verdict;

#[cfg(test)]
mod board_tests;
#[cfg(test)]
mod facts_tests;
#[cfg(test)]
mod nym_facts_tests;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod report_tests;
#[cfg(test)]
mod verdict_tests;
