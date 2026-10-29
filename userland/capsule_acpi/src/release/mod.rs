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

//! What this build runs: everything, or with `release-power-only` (the
//! Standard, Hardened and Air-Gapped images, Capsule.mk) only the battery,
//! AC, lid and button path and what only reads (`rule`).

pub mod rule;

pub use rule::Part;

pub const POWER_ONLY: bool = cfg!(feature = "release-power-only");

pub fn on(part: Part) -> bool {
    rule::runs(POWER_ONLY, part)
}

pub fn op_on(op: u8) -> bool {
    rule::op_runs(POWER_ONLY, op)
}

/// One line at start, so a log says which build ran.
pub fn say() {
    let what: &[u8] = if POWER_ONLY { b"power only" } else { b"full" };
    crate::sys::log::Line::new(b"build: ").str(what).end();
}
