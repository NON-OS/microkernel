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

//! A port-only claim (H-02, src/hardware/broker/confine/port_only.rs): a
//! capsule holding neither Dma nor Mmio neither joins an isolation group
//! nor is blocked by one, in either claim order, but across a bridge to a
//! shared conventional bus; a capsule that can DMA, holding a sibling,
//! blocks as before.

use super::member::Member;
use super::port_only::{blocker, blocks, port_only};

/// A sibling function of the claimed device (no ACS), or a device across a
/// shared conventional bus.
const SIBLING: Member = Member { id: 7, at: (0, 0x1f, 3), hop: (0, 0x1f, 2), shared: false };
const ACROSS: Member = Member { id: 8, at: (6, 2, 0), hop: (2, 3, 0), shared: true };

#[test]
fn port_only_is_neither_dma_nor_mmio() {
    assert!(port_only(false, false));
    assert!(!port_only(false, true), "a capsule holding Mmio is not port-only");
    assert!(!port_only(true, false), "a capsule holding Dma is not port-only");
    assert!(!port_only(true, true));
}

#[test]
fn a_port_only_holder_never_blocks_a_siblings_claim_in_either_order() {
    assert!(!blocks(false, true, false), "SMBus held first, AHCI claims");
    assert!(!blocks(true, false, false), "AHCI held first, SMBus claims");
    assert!(!blocks(true, true, false), "two port-only claims");
    let held = |id| id == SIBLING.id;
    assert!(blocker(&[SIBLING], held, false, true).is_none());
    assert!(blocker(&[SIBLING], held, true, false).is_none());
}

#[test]
fn across_a_shared_conventional_bus_a_port_only_holder_still_blocks() {
    for (claim, holder) in [(true, false), (false, true), (true, true), (false, false)] {
        assert!(blocks(claim, holder, true), "claim {claim} holder {holder}");
    }
    let held = |id| id == ACROSS.id;
    assert_eq!(blocker(&[SIBLING, ACROSS], held, true, false).map(|m| m.id), Some(ACROSS.id));
}

#[test]
fn a_capsule_that_can_dma_holding_a_sibling_still_blocks() {
    assert!(blocks(false, false, false));
    let held = |id| id == SIBLING.id;
    assert_eq!(blocker(&[SIBLING], held, false, false).map(|m| m.at), Some(SIBLING.at));
    assert!(blocker(&[SIBLING], |_| false, false, false).is_none(), "a member nobody holds");
}
