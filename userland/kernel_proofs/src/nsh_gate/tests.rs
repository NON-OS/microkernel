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

const LIB: &str = include_str!("../../../../src/lib.rs");
const CARGO: &str = include_str!("../../../../Cargo.toml");
const PROFILES: &str = include_str!("../../../../tools/nix/config.nix");
const SPAWN: &str = include_str!("../../../../src/userspace/capsule_nsh_attest/spawn.rs");
const ATTEST_MK: &str = include_str!("../../../capsule_nsh_attest/Capsule.mk");
const FRONT_SPAWN: &str = include_str!("../../../../src/userspace/capsule_nsh_front/spawn.rs");
const FRONT_MK: &str = include_str!("../../../capsule_nsh_front/Capsule.mk");

const NETWORK: u64 = 1 << 2;

fn profile(name: &str) -> &'static str {
    let at = PROFILES.find(&format!("    {name} = {{")).expect("the profile");
    let rest = &PROFILES[at..];
    &rest[..rest.find("\n    };").expect("its end")]
}

/// The `cfg(all(...))` the compile_error! carrying `text` sits under.
fn gate_of(text: &str) -> &'static str {
    let at = LIB.find(text).expect("the refusal");
    let cfg = LIB[..at].rfind("#[cfg(").expect("its cfg");
    &LIB[cfg..at]
}

fn mask(src: &str, key: &str) -> u64 {
    let line = src.lines().find(|l| l.starts_with(key)).unwrap_or_else(|| panic!("{key}"));
    let hex: String = line
        .rsplit("0x")
        .next()
        .expect("a mask")
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    u64::from_str_radix(&hex, 16).expect("hex")
}

#[test]
fn a_release_kernel_carries_the_nsh_capsules_only_through_nonos_server() {
    let gate = gate_of("ship in the server profile alone");
    for part in [
        "feature = \"nonos-release\"",
        "all(feature = \"nonos-production\", not(feature = \"nonos-dev-attest\"))",
        "not(feature = \"nonos-server\")",
        "any(feature = \"nonos-capsule-nsh-front\", feature = \"nonos-capsule-nsh-attest\")",
    ] {
        assert!(gate.contains(part), "the gate holds {part}: {gate}");
    }
    assert!(gate.starts_with("#[cfg(all("), "every part must hold");
    let pair = gate_of("so it needs nonos-capsule-nsh-attest");
    assert!(pair.contains(
        "all(feature = \"nonos-capsule-nsh-front\", not(feature = \"nonos-capsule-nsh-attest\"))"
    ));
}

#[test]
fn nonos_server_turns_on_both_capsules_and_only_the_standard_and_server_profiles_name_it() {
    assert!(CARGO.contains(
        "\nnonos-server = [\"nonos-capsule-nsh-attest\", \"nonos-capsule-nsh-front\"]\n"
    ));
    assert!(profile("server").contains("kernel = [ \"microkernel-full-gui\" \"nonos-server\" ];"));
    assert!(profile("server").contains("loader = \"standard\";"));
    assert!(profile("server").contains("drop = debugFeatures;"), "no capsule writes to serial");
    assert!(profile("standard").contains("kernel = [ \"microkernel-full-gui\" \"nonos-server\" ];"));
    assert!(profile("standard").contains("loader = \"standard\";"));
    for desktop in ["hardened", "airgapped", "qemu", "dev", "selftest", "gauntlet", "core"] {
        assert!(
            !profile(desktop).contains("nsh") && !profile(desktop).contains("nonos-server"),
            "{desktop}"
        );
    }
    let refused =
        PROFILES.find("the nsh capsules ship in profile server").expect("the profile check");
    let check = &PROFILES[PROFILES[..refused].rfind("assert").expect("its assert")..refused];
    assert!(
        check.contains("builtins.elem \"nonos-server\" enabled || loader == \"dev-qemu\""),
        "{check}"
    );
}

#[test]
fn nsh_attest_asks_for_what_it_declares_and_never_the_network() {
    assert!(SPAWN.contains("const REQUIRED_CAPS: u64 = 0x8000_0079;"));
    assert!(
        SPAWN.contains("requested_caps: REQUIRED_CAPS | crate::capabilities::serial_debug_cap(),")
    );
    let declared = mask(ATTEST_MK, "CAPSULE_REQUIRED_CAPS");
    assert_eq!(declared, 0x8000_0079);
    assert_eq!(declared & NETWORK, 0, "nsh.attest never reaches the network");
    assert_eq!(mask(ATTEST_MK, "CAPSULE_OPTIONAL_CAPS"), 0x100, "Debug, a development twin's");
}

#[test]
fn nsh_front_asks_for_the_network_and_nothing_that_holds_a_key_or_a_quote() {
    assert!(FRONT_SPAWN.contains("const REQUIRED_CAPS: u64 = 0x0000_001D;"));
    let declared = mask(FRONT_MK, "CAPSULE_REQUIRED_CAPS");
    assert_eq!(declared, 0x1D);
    const CRYPTO: u64 = 1 << 5;
    const FILE_SYSTEM: u64 = 1 << 6;
    const ATTEST_READ: u64 = 1 << 31;
    assert_eq!(declared & (CRYPTO | FILE_SYSTEM | ATTEST_READ), 0);
    assert_eq!(declared & NETWORK, NETWORK);
}
