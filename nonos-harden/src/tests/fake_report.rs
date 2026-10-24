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

//! A nonos-verify report as the module writes it (nonos-verify/src/report.rs),
//! with the checks and gaps a proof sets.

use serde_json::{json, Value};

/// The declared list the fixture tree holds: the one gap a proof may state
/// while trust-chain still passes.
pub const DECLARED: &[u8] =
    b"# format: nonos-harden-declared-limits/1\ntrust-chain: field asserts\n";

pub fn report(checks: &[(&str, &str)], gaps: &[(&str, &str)]) -> Value {
    let checks: Vec<Value> = checks
        .iter()
        .map(|(n, s)| json!({"name": n, "status": s, "detail": "as set", "artifact": ""}))
        .collect();
    let gaps: Vec<Value> = gaps.iter().map(|(w, n)| json!({"what": w, "needs": n})).collect();
    json!({"module": "m", "commit": "c", "status": "pass", "blocking": true, "checks": checks, "gaps": gaps})
}

/// The gap trust-chain declares, stated by trust-chain alone.
pub fn gap(sub: &str) -> Option<Value> {
    let gaps: &[_] =
        if sub == "trust-chain" { &[("field asserts", "exposed fields")] } else { &[] };
    Some(report(&[("a", "pass")], gaps))
}
pub fn other_gap(sub: &str) -> Option<Value> {
    let gaps: &[_] =
        if sub == "trust-chain" { &[("field asserts", "x"), ("a new gap", "y")] } else { &[] };
    Some(report(&[("a", "pass")], gaps))
}
// The gap trust-chain declares, stated by every module.
pub fn gap_everywhere(_: &str) -> Option<Value> {
    Some(report(&[("a", "pass")], &[("field asserts", "exposed fields")]))
}
