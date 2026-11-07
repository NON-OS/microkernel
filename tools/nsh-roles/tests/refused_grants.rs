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

//! Grants and approvals held to the table: an unnamed role, a wrong anchor,
//! an escalation over an hour, and approvals from a key that is not an
//! approver, for a base grant, or given twice.

mod common;
#[path = "common/refuse.rs"]
mod refuse;
use common::{fleet, fresh, ok};
use refuse::refused;

#[test]
fn grants_are_held_to_the_table() {
    let d = fresh("grant");
    let anchor = fleet(&d);
    let t = ["--table", "table", "--anchor", anchor.as_str()];
    let who = ["--subject", "user.public", "--user", "ek", "--key", "auth.secret"];
    let g = |role: &'static str, h: &'static str, out: &'static str| {
        [&["grant"][..], &t, &who, &["--role", role, "--cap", "IPC", "--hours", h, "--out", out]]
            .concat()
    };
    refused(&d, &g("nobody", "1", "g1"), "role", "g1");
    let wrong = ["--table", "table", "--anchor", "00".repeat(32).leak()];
    let a = [
        &["grant"][..],
        &wrong,
        &who,
        &["--role", "operator", "--cap", "IPC", "--hours", "1", "--out", "g2"],
    ];
    refused(&d, &a.concat(), "Signer", "g2");
    ok(&d, &g("operator", "4", "base"));
    let id = nsh_enrol::hex::of(
        nsh_wire::Grant::parse(&std::fs::read(d.join("base")).expect("base"))
            .expect("grant")
            .grant_id,
    );
    let esc = |h: &'static str, out: &'static str| {
        [g("operator", h, out), vec!["--parent", id.clone().leak()]].concat()
    };
    refused(&d, &esc("2", "e1"), "one hour", "e1");
    ok(&d, &esc("1", "esc"));
    let ap = |key: &'static str, grant: &'static str, out: &'static str| {
        [&["approve"][..], &t, &["--grant", grant, "--key", key, "--out", out]].concat()
    };
    refused(&d, &ap("user.secret", "esc", "a1"), "Approval", "a1");
    refused(&d, &ap("approver.secret", "base", "a2"), "only an escalation", "a2");
    ok(&d, &ap("approver.secret", "esc", "esc1"));
    refused(&d, &ap("approver.secret", "esc1", "a3"), "already", "a3");
    refused(&d, &ap("approver.secret", "esc", "esc1"), "File exists", "a4");
}
