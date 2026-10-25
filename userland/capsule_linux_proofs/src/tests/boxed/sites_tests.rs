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

//! Every place the personality refuses an action records it for the box:
//! a refusal site that stops recording drops refusals from every report.

macro_rules! src {
    ($p:literal) => {
        ($p, include_str!(concat!("../../../../capsule_linux/src/linux/", $p)))
    };
}

const SITES: &[((&str, &str), &str)] = &[
    (src!("app/gate.rs"), "boxed::of_needs(needs)"),
    (src!("app/calls.rs"), "Kind::Call"),
    (src!("serve/refused.rs"), "Kind::Barred"),
    (src!("serve/unserved.rs"), "Kind::Unserved"),
    (src!("net/socket_kind.rs"), "Kind::Network"),
    (src!("net/offline.rs"), "Kind::Network"),
    (src!("file/root_write.rs"), "Kind::Write"),
    (src!("file/xattrs/xattr/answer.rs"), "Kind::Write"),
];

const POLICY: &str = include_str!("../../../../capsule_linux/src/linux/net/policy.rs");

#[test]
fn every_refusal_site_records_its_kind() {
    for ((name, text), kind) in SITES {
        assert!(text.contains(kind), "{name} no longer records {kind}");
    }
    // Both policy refusals of a datagram or a bind, each once.
    assert_eq!(POLICY.matches("Kind::Network").count(), 2);
}

#[test]
fn a_write_is_counted_where_it_is_done_not_where_it_is_asked() {
    let access = include_str!("../../../../capsule_linux/src/linux/file/meta/query/access.rs");
    assert!(access.contains(".may_write()") && !access.contains(".writable()"));
}
