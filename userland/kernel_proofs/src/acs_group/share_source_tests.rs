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

//! The shared group's wiring, read where it lives: the claim asks before it
//! refuses, an alias is never shared, and only a standard image carries it.

const SHARE: &str = include_str!("../../../../src/hardware/broker/confine/group_share.rs");
const JOIN: &str = include_str!("../../../../src/hardware/broker/confine/join_group.rs");
const CARGO: &str = include_str!("../../../../Cargo.toml");
const PROFILES: &str = include_str!("../../../../tools/nix/config.nix");

#[test]
fn the_claim_asks_before_it_refuses_and_an_alias_is_never_shared() {
    let at = JOIN.find("blocked.filter(|b| !shares(all[pos].pid, c.pid, b))").expect("the ask");
    assert!(JOIN[at..].find("say(line.as_bytes());").is_some(), "the refusal comes after the ask");
    let alias = JOIN.find("aliases.iter().find(").expect("the alias check");
    assert!(alias > at && !JOIN[alias..].contains("shares("), "an alias is shared");
    assert!(SHARE.contains("let reaches = matches!(b, Blocked::Reaches(..));"));
    assert!(
        SHARE.contains("matches!(boot_profile(), BootProfile::Hardened | BootProfile::AirGapped)")
    );
    assert!(
        SHARE.contains(".iter().all(|&p| authority_of(p) == Some(Authority::Vendor))"),
        "both, not either"
    );
}

#[test]
fn a_standard_image_carries_the_posture_and_a_hardened_or_airgapped_one_drops_it() {
    let gui = &CARGO[CARGO.find("\nmicrokernel-desktop-gui = [").expect("the set")..];
    assert!(gui[..gui.find(']').expect("its end")].contains("\"nonos-shared-group-posture\""));
    let posture =
        "postureFeatures = [ \"nonos-dma-unconfined-posture\" \"nonos-shared-group-posture\" ];";
    assert!(PROFILES.contains(posture));
    for dropped in ["hardened", "airgapped"] {
        let at = PROFILES.find(&format!("    {dropped} = {{")).expect("the profile");
        let p = &PROFILES[at..];
        assert!(p[..p.find("\n    };").unwrap()].contains("++ postureFeatures"), "{dropped}");
    }
}
