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

//! The world domain against wireless-regdb's `country 00`, channel by
//! channel, and the channel numbers against IEEE Std 802.11-2020 Annex E.

use nonos_wifi_core::regdom::{channel_mhz, Band, Permit, WORLD};

const PASSIVE: Permit = Permit::Passive { dfs: false };
const RADAR: Permit = Permit::Passive { dfs: true };

#[test]
fn the_world_domain_opens_only_channels_1_to_11_for_probing() {
    for ch in 1..=11 {
        assert_eq!(WORLD.permit(Band::Ghz2, ch), Permit::Active, "channel {ch}");
    }
    assert_eq!(WORLD.permit(Band::Ghz2, 12), PASSIVE);
    assert_eq!(WORLD.permit(Band::Ghz2, 13), PASSIVE);
    // 802.11b only, and this station runs OFDM.
    assert_eq!(WORLD.permit(Band::Ghz2, 14), Permit::Disabled);
    assert_eq!(WORLD.permit(Band::Ghz2, 15), Permit::Disabled);
}

#[test]
fn the_world_domain_leaves_5ghz_passive_marks_radar_and_closes_6ghz() {
    for ch in [36, 40, 44, 48, 149, 153, 157, 161, 165] {
        assert_eq!(WORLD.permit(Band::Ghz5, ch), PASSIVE, "channel {ch}");
    }
    for ch in [52, 56, 60, 64, 100, 116, 132, 140, 144] {
        assert_eq!(WORLD.permit(Band::Ghz5, ch), RADAR, "channel {ch}");
    }
    for ch in [32, 68, 96, 169, 173, 177] {
        assert_eq!(WORLD.permit(Band::Ghz5, ch), Permit::Disabled, "channel {ch}");
    }
    for ch in [1, 5, 37, 93, 233] {
        assert_eq!(WORLD.permit(Band::Ghz6, ch), Permit::Disabled, "6 GHz {ch}");
    }
}

#[test]
fn power_is_the_rule_ceiling_and_absent_where_closed() {
    assert_eq!(WORLD.max_eirp_dbm(Band::Ghz2, 6), Some(20));
    assert_eq!(WORLD.max_eirp_dbm(Band::Ghz5, 100), Some(20));
    assert_eq!(WORLD.max_eirp_dbm(Band::Ghz2, 14), None);
}

#[test]
fn channel_numbers_name_annex_e_frequencies() {
    assert_eq!(channel_mhz(Band::Ghz2, 1), Some(2412));
    assert_eq!(channel_mhz(Band::Ghz2, 13), Some(2472));
    assert_eq!(channel_mhz(Band::Ghz2, 14), Some(2484));
    assert_eq!(channel_mhz(Band::Ghz2, 0), None);
    assert_eq!(channel_mhz(Band::Ghz5, 36), Some(5180));
    assert_eq!(channel_mhz(Band::Ghz5, 165), Some(5825));
    assert_eq!(channel_mhz(Band::Ghz6, 1), Some(5955));
    assert_eq!(channel_mhz(Band::Ghz6, 2), Some(5935));
    assert_eq!(channel_mhz(Band::Ghz6, 233), Some(7115));
    assert_eq!(channel_mhz(Band::Ghz6, 3), None);
    assert_eq!(Band::of_legacy(14), Band::Ghz2);
    assert_eq!(Band::of_legacy(36), Band::Ghz5);
}
