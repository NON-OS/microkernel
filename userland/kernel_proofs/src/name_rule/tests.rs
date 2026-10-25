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

//! A store capsule cannot take a system service's name, at any time; the
//! service's own signed capsule can; a publisher stays in its namespace.

use super::baked_apps::APPS;
use super::baked_names::{CORE, FAMILIES};
use super::rule::{admit_loaded, NameRefusal};

pub(super) fn admit(name: &str, reply: bool, ns: &str, publisher: bool) -> Result<(), NameRefusal> {
    admit_loaded(name, reply, ns, publisher, &[CORE, APPS], FAMILIES)
}

const STORE: &str = "com.example.game_2048";

#[test]
fn a_store_capsule_never_takes_the_compositors_name() {
    // The rule reads no liveness: compositor alive, in its zombie window, or
    // past its restart budget, the same declaration gets the same answer.
    for _when in ["alive", "zombie window", "budget spent"] {
        assert_eq!(admit("compositor", false, STORE, true), Err(NameRefusal::Reserved));
        let reply = admit("endpoint.compositor.reply", true, STORE, true);
        assert_eq!(reply, Err(NameRefusal::Reserved));
    }
    let other_nonos = admit("compositor", false, "systems.nonos.app.hello", false);
    assert_eq!(other_nonos, Err(NameRefusal::Reserved), "another of ours, too");
}

#[test]
fn the_services_own_signed_capsule_takes_its_name_back() {
    assert_eq!(admit("compositor", false, "systems.nonos.compositor", false), Ok(()));
    assert_eq!(admit("endpoint.compositor.reply", true, "systems.nonos.compositor", false), Ok(()));
    let child = admit("app.linux.term.9", false, "systems.nonos.app.linux", false);
    assert_eq!(child, Ok(()), "a name under its own");
    let crypto = admit("crypto_pool", false, "systems.nonos.crypto", false);
    assert_eq!(crypto, Ok(()), "by its table's namespace, not its name's");
    let spoof = admit("compositor", false, "systems.nonos.compositor", true);
    assert_eq!(spoof, Err(NameRefusal::Reserved), "never a publisher");
}

#[test]
fn a_publisher_declares_only_names_under_its_namespace() {
    assert_eq!(admit(STORE, false, STORE, true), Ok(()));
    assert_eq!(admit("com.example.game_2048.worker", false, STORE, true), Ok(()));
    assert_eq!(admit("endpoint.com.example.game_2048.reply", true, STORE, true), Ok(()));
    let outside = Err(NameRefusal::OutsideNamespace);
    assert_eq!(admit("app.game_2048", false, STORE, true), outside);
    assert_eq!(
        admit("com.example.game_2048x", false, STORE, true),
        outside,
        "a label, not a prefix"
    );
    assert_eq!(admit("com.example.other", false, STORE, true), outside);
}
