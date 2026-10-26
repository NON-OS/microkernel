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

//! The owner's O-IOMMU ruling: with no IOMMU in service, a Hardened or
//! Air-Gapped boot and an AgentProof build stop, each with its line; every
//! other boot goes on, and any of them goes on with a unit in service.

use super::no_iommu_refusal::{stops, Needs};

const B: [bool; 2] = [false, true];

#[test]
fn each_promise_stops_a_boot_with_no_unit() {
    assert_eq!(stops(false, false, false, true), Some(Needs::AirGapped));
    assert_eq!(stops(false, false, true, false), Some(Needs::Hardened));
    assert_eq!(stops(false, true, false, false), Some(Needs::AgentProof));
    assert_eq!(stops(false, false, false, false), None, "Standard goes on, marked");
}

#[test]
fn a_unit_in_service_lets_every_boot_go_on() {
    for agentproof in B {
        for hardened in B {
            for air_gapped in B {
                assert_eq!(stops(true, agentproof, hardened, air_gapped), None);
            }
        }
    }
}

#[test]
fn the_boot_menu_choice_is_named_before_the_build() {
    assert_eq!(stops(false, true, true, true), Some(Needs::AirGapped));
    assert_eq!(stops(false, true, true, false), Some(Needs::Hardened));
    assert_eq!(stops(false, false, true, true), Some(Needs::AirGapped), "air-gapped is hardened");
}

#[test]
fn each_line_says_who_needs_what_and_why() {
    let tail = " needs an IOMMU to confine device DMA; this machine has none";
    assert_eq!(Needs::AirGapped.line(), format!("air-gapped{tail}"));
    assert_eq!(Needs::Hardened.line(), format!("hardened{tail}"));
    assert_eq!(Needs::AgentProof.line(), format!("AgentProof{tail}"));
}
