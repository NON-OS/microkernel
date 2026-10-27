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

const TAIL: &str = include_str!("../../../../src/sys/serial/tail.rs");
const CARGO: &str = include_str!("../../../../Cargo.toml");
const PROFILES: &str = include_str!("../../../../tools/nix/config.nix");

fn profile(name: &str) -> &'static str {
    let at = PROFILES.find(&format!("    {name} = {{")).expect("the profile");
    let rest = &PROFILES[at..];
    &rest[..rest.find("\n    };").expect("its end")]
}

#[test]
fn the_tail_keeps_bytes_only_with_one_of_its_two_features() {
    let gate = "if !cfg!(any(feature = \"nonos-log-tail\", feature = \"capsule-serial-debug\")) {";
    let at = TAIL.find(gate).expect("the gate");
    assert!(
        TAIL[..at].contains("pub(super) fn keep(b: u8) {"),
        "the gate is not keep's first test"
    );
    assert!(
        TAIL[at..].starts_with(&format!("{gate}\n        return;")),
        "the gate does not return"
    );
}

#[test]
fn a_standard_image_keeps_the_tail_and_a_hardened_or_airgapped_one_does_not() {
    assert!(CARGO.contains("\nnonos-log-tail = []\n"), "the feature");
    let gui = &CARGO[CARGO.find("\nmicrokernel-desktop-gui = [").expect("the set")..];
    assert!(gui[..gui.find(']').expect("its end")].contains("\"nonos-log-tail\""));
    assert!(PROFILES.contains("tailFeatures = [ \"nonos-log-tail\" ];"));
    for kept in ["standard", "qemu", "dev"] {
        assert!(!profile(kept).contains("tailFeatures"), "{kept} drops the tail");
    }
    for dropped in ["hardened", "airgapped"] {
        assert!(profile(dropped).contains("++ tailFeatures"), "{dropped} keeps the tail");
    }
    assert!(profile("standard").contains("drop = debugFeatures;"), "standard lets capsules write");
}
