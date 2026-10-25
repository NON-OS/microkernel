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

//! The network cards are one table, and the kernel reads it everywhere it
//! names a card.

use super::common::{WIFI, WIRED};
use super::held::callers_of;
use super::nics::{Link, NICS};

/// The cards keep the names they have always registered, in order: init's
/// notes, the attack suite's report and the capsules' own logs say them.
#[test]
fn the_cards_keep_their_names() {
    let of = |link| NICS.iter().filter(|(_, l)| *l == link).map(|(c, _)| *c).collect::<Vec<_>>();
    assert_eq!(of(Link::Wired), WIRED);
    assert_eq!(of(Link::Wifi), WIFI);
}

/// Each card on the table is the name its spawn registers, held, raised to
/// the packet band and refused by a profile with no network.
#[test]
fn the_table_names_every_card_the_kernel_starts() {
    let refuse = include_str!(
        "../../../../src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs"
    );
    let priority = include_str!(
        "../../../../src/kernel_core/process_spawn/capsule_spawn/runner/install/priority.rs"
    );
    for (file, src) in [("profile_refuse.rs", refuse), ("priority.rs", priority)] {
        assert!(src.contains("nics::NICS.iter()"), "{file} does not read the card table");
    }
    // In the table's order.
    let spawns = [
        include_str!("../../../../src/hardware/virtio_net_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/e1000_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/e1000e_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/igc_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rtl8169_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rtl8139_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/igb_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/ixgbe_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/pcnet_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/vmxnet3_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/tulip_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/cdc_ecm_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/cdc_ncm_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rndis_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/iwlwifi_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rtl8821ce_capsule/spawn.rs"),
    ];
    assert_eq!(spawns.len(), NICS.len(), "a card on the table has no spawn here");
    for ((card, _), spawn) in NICS.iter().zip(spawns) {
        let line = format!("const SERVICE_NAME: &str = \"{card}\";");
        assert!(spawn.contains(&line), "{card} is not the name its spawn registers");
        assert!(callers_of(card).is_some(), "{card} is a network driver and not held");
    }
}
