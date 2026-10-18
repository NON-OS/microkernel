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

//! The families the kernel trusts by name and its own inboxes; every
//! reserved entry admits its owner's namespace alone.

use super::baked_apps::APPS;
use super::baked_names::CORE;
use super::rule::NameRefusal;
use super::tests::admit;

#[test]
fn a_family_member_is_its_own_capsules_alone() {
    let nvme = "systems.nonos.driver.nvme0";
    assert_eq!(admit("driver.nvme0", false, nvme, false), Ok(()));
    assert_eq!(admit("endpoint.driver.nvme0.reply", true, nvme, false), Ok(()));
    let refused = Err(NameRefusal::Reserved);
    assert_eq!(admit("driver.nvme0", false, "systems.nonos.app.hello", false), refused);
    assert_eq!(
        admit("driver.new_one0", false, "com.example.x", true),
        refused,
        "unbaked, still the family's"
    );
    assert_eq!(
        admit("net.tcp", false, "systems.nonos.net.tcp", true),
        refused,
        "never a publisher"
    );
    assert_eq!(admit("net.dhcp.client", false, "systems.nonos.net.dhcp.client", false), Ok(()));
}

#[test]
fn each_reserved_name_admits_exactly_its_namespace() {
    for &(name, ns) in CORE.iter().chain(APPS) {
        let own = admit(name, false, ns, false);
        assert_eq!(own.is_ok(), !ns.is_empty(), "{name} under {ns:?}");
        assert!(admit(name, false, "systems.nonos.someone_else", false).is_err(), "{name}");
        assert!(admit(name, false, ns, true).is_err(), "{name} as a publisher");
    }
}

#[test]
fn the_kernels_inboxes_are_nobodys() {
    for (name, reply) in [("endpoint.4294967299", true), ("proc.5", false), ("endpoint.x", false)] {
        assert_eq!(admit(name, reply, "systems.nonos.x", false), Err(NameRefusal::Reserved));
    }
    assert_eq!(
        admit("app.prove.job", false, "", false),
        Err(NameRefusal::Reserved),
        "no namespace owns it"
    );
}
