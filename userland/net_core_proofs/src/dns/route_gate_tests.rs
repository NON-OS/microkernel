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

//! A name goes out in the clear only on the Direct route: the rule over every
//! value the policy store can give, and its place in RESOLVE_A, read from the
//! source.

use nonos_policy_proto::route;

use super::route_gate::clear_lookup_allowed;

const RESOLVE_A: &str =
    include_str!("../../../capsule_net_core/src/server/handlers/dns/resolve_a.rs");

#[test]
fn only_the_direct_route_resolves_in_the_clear() {
    assert!(clear_lookup_allowed(Some(route::DIRECT)));
    assert!(!clear_lookup_allowed(None), "a store that does not answer is not Direct");
    for v in (0..=u8::MAX).filter(|v| *v != route::DIRECT) {
        assert!(!clear_lookup_allowed(Some(v)), "route {v}");
    }
}

#[test]
fn resolve_a_asks_the_gate_before_any_query_starts() {
    let gate = RESOLVE_A.find("if !super::route_gate::clear_lookup_allowed(chosen) {");
    let refusal = RESOLVE_A.find("return err(sender_pid, req.request_id, E_NOT_DIRECT, tx);");
    let query = RESOLVE_A.find("s.start_query(");
    assert!(matches!((gate, refusal, query), (Some(g), Some(r), Some(q)) if g < r && r < q));
    assert!(RESOLVE_A.contains("get_u8(port, Field::NetworkRoute)"));
}
