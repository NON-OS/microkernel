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

//! The Country element narrows a domain and never widens it, and a sweep
//! probes only where the domain allows.

use nonos_wifi_core::regdom::{parse_country, Band, Permit, ScanMode, ScanPlan, WORLD};

// "DE", any environment: 1-13, 36-48, 52-64, behind an operating extension.
const DE: &[u8] = &[b'D', b'E', b' ', 201, 81, 0, 1, 13, 20, 36, 4, 23, 52, 4, 20];

#[test]
fn a_country_element_lists_its_channels() {
    let c = parse_country(DE).unwrap();
    assert_eq!((c.alpha2, c.environment), (*b"DE", b' '));
    for ch in (1..=13).chain([36, 40, 44, 48, 52, 56, 60, 64]) {
        assert!(c.channels.contains(ch), "channel {ch}");
    }
    for ch in [14, 32, 68, 100, 149] {
        assert!(!c.channels.contains(ch), "channel {ch}");
    }
}

#[test]
fn an_element_that_names_nothing_restricts_nothing() {
    assert!(parse_country(b"DE").is_none());
    assert!(parse_country(&[b'D', b'E', b' ', 201, 81, 0]).is_none());
    // A run that steps past channel 255 stops there.
    let c = parse_country(&[b'X', b'X', b'X', 249, 3, 20]);
    assert!(c.is_none(), "249 is an operating extension");
    assert!(parse_country(&[b'Z', b'Z', b' ', 177, 40, 20]).unwrap().channels.contains(253));
}

#[test]
fn a_country_narrows_the_world_domain_and_never_widens_it() {
    let d = WORLD.restrict(&parse_country(DE).unwrap());
    assert_eq!(d.permit(Band::Ghz2, 1), Permit::Active);
    assert_eq!(d.permit(Band::Ghz2, 13), Permit::Passive { dfs: false });
    assert_eq!(d.permit(Band::Ghz5, 149), Permit::Disabled);
    assert_eq!(d.permit(Band::Ghz5, 100), Permit::Disabled);
    let jp = parse_country(&[b'J', b'P', b' ', 1, 14, 20]).unwrap();
    assert_eq!(WORLD.restrict(&jp).permit(Band::Ghz2, 14), Permit::Disabled);
    // Two countries narrow to what both list.
    let only_1 = parse_country(&[b'Q', b'Q', b' ', 1, 1, 20]).unwrap();
    assert_eq!(d.restrict(&only_1).permit(Band::Ghz2, 2), Permit::Disabled);
}

#[test]
fn a_world_sweep_probes_1_to_11_and_listens_elsewhere() {
    let steps: Vec<_> = ScanPlan::new(WORLD, true, true).collect();
    let active: Vec<u8> =
        steps.iter().filter(|s| s.mode == ScanMode::Active).map(|s| s.channel).collect();
    assert_eq!(active, (1..=11).collect::<Vec<u8>>());
    assert!(steps.iter().all(|s| s.channel != 14 && s.channel != 169));
    assert_eq!(steps.len(), 13 + 25);
    let ch36 = steps.iter().find(|s| s.channel == 36).unwrap();
    assert_eq!((ch36.mhz, ch36.mode, ch36.dwell_ms), (5180, ScanMode::Passive, 120));
    assert_eq!(steps[0].dwell_ms, 40);
    assert_eq!(ScanPlan::new(WORLD, true, false).count(), 13);
    assert_eq!(ScanPlan::new(WORLD, false, false).count(), 0);
}
