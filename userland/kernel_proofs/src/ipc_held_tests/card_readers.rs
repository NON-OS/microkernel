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

//! The capsules that name a network card read the kernel's table.

/// The capsules that look the cards up mount the kernel's table itself, so
/// net.core, net.l2, the setup wizard and the attack suite name the same
/// cards the kernel holds.
#[test]
fn the_capsules_read_the_kernels_table() {
    let mount = "src/services/registry/nics.rs\"]";
    let readers = [
        ("net.core", include_str!("../../../capsule_net_core/src/setup/candidates.rs")),
        ("net.l2", include_str!("../../../capsule_net_l2/src/setup/discover.rs")),
        ("the wizard", include_str!("../../../capsule_setup_wizard/src/network/wired.rs")),
        ("capsule_attack", include_str!("../../../capsule_attack/src/attempts.rs")),
    ];
    for (who, src) in readers {
        assert!(src.contains(mount), "{who} keeps its own list of cards");
        for card in ["\"driver.e1000_0\"", "\"driver.cdc_ecm0\""] {
            assert!(!src.contains(card), "{who} still names {card} itself");
        }
    }
}
