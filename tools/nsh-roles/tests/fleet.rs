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

//! The whole fleet, made with the command line and checked by the host's
//! own code: the grant `grant` writes logs in through `nsh_roles::login`,
//! the escalation `approve` completes passes `nsh_roles::escalate`, and a
//! list from `revoke` withdraws the grant.

mod common;
#[path = "common/host.rs"]
mod host;
use common::{fleet, fresh, ok};
use host::{host_login, read};
use nsh_roles::{escalate, open_policy, open_revocations, Host, Refusal};
use nsh_wire::Grant;

fn hex(b: &[u8]) -> String {
    nsh_enrol::hex::of(b)
}

#[test]
fn a_fleet_made_here_is_one_a_host_accepts() {
    let d = fresh("fleet");
    let anchor = fleet(&d);
    let a: [u8; 32] = nsh_enrol::hex::array(&anchor, "anchor").expect("anchor");
    let t = ["--table", "table", "--anchor", anchor.as_str()];
    ok(&d, &[&["revoke"][..], &t, &["--key", "auth.secret", "--out", "list"]].concat());
    let base =
        [&t[..], &["--subject", "user.public", "--user", "ek", "--role", "operator"]].concat();
    let caps = ["--cap", "CoreExec", "--cap", "IPC", "--hours", "4", "--key", "auth.secret"];
    ok(&d, &[&["grant"][..], &base, &caps, &["--out", "base"]].concat());
    let l = host_login(&d, &a, "list").expect("the grant logs in");
    assert_eq!(l.user, b"ek");
    let base_id = hex(Grant::parse(&read(&d, "base")).expect("grant").grant_id);
    let esc =
        ["--cap", "CoreExec", "--cap", "Network", "--hours", "1", "--parent", base_id.as_str()];
    let key = ["--key", "auth.secret", "--out", "esc"];
    ok(&d, &[&["grant"][..], &base, &esc, &key].concat());
    let appr = ["--grant", "esc", "--key", "approver.secret", "--out", "esc.approved"];
    let shown = ok(&d, &[&["approve"][..], &t, &appr].concat());
    assert!(shown.contains("escalation grant") && shown.contains("approvals 1"), "{shown}");
    let (tb, lb) = (read(&d, "table"), read(&d, "list"));
    let p = open_policy(&tb, &a, 0).expect("table");
    let r = open_revocations(&lb, &p, 0).expect("list");
    let host = Host::new(&p, &r, nsh_enrol::files::now_ms().expect("clock")).expect("host");
    let g = read(&d, "esc.approved");
    let e = escalate(&host, &l, false, &Grant::parse(&g).expect("grant")).expect("escalates");
    assert!(e.caps.contains(b"Network"));
    ok(
        &d,
        &[&["revoke"][..], &t, &["--id", &base_id, "--key", "auth.secret", "--out", "list2"]]
            .concat(),
    );
    assert_eq!(host_login(&d, &a, "list2").err(), Some(Refusal::Revoked));
}
