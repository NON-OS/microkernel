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

use nonos_attest_path::Kind;

/// A v4 trailer checked as a gate checks it: a strict parse for this kind, the
/// path folded to `root`, then the proof over words computed from `ctx`. The
/// check is the proof verifier's (nonos-proof-verify), so what this tool
/// enrolls and what anyone re-checks off the machine is one implementation.
/// A development enrollment's trailer is the path alone (policy::paths_only).
pub fn check_v4(root: &[u8; 32], kind: Kind, ctx: &[u8], t: &[u8]) -> Result<(), String> {
    nonos_proof_verify::check_v4(root, kind, ctx, t, crate::policy::paths_only())
        .map(drop)
        .map_err(|r| r.reason())
}
