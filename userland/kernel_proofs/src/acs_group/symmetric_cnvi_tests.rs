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

//! The CNVi shape: a function without ACS (the PCH xHCI) beside one
//! that redirects (CNVi Wi-Fi, an Intel RCiEP) reaches the other one-sidedly,
//! and the claim is refused in both orders, not only the one K11b caught;
//! and the line for a peer carrying the claimed device's requester id.

use super::acs_refusal::peer_alias_line;
use super::acs_tree::{Bdf, Func};
use super::member::Member;
use super::symmetric_tests::refused;

#[test]
fn the_cnvi_shape_is_refused_whichever_claims_first() {
    let xhci = Func { at: (0, 0x14, 0), buses: None, conventional: false, alias: None };
    let cnvi = Func { at: (0, 0x14, 3), ..xhci };
    let funcs = vec![xhci, cnvi];
    // The xHCI offers no ACS; CNVi redirects as an Intel RCiEP does.
    let acs = |b: Bdf, _: u16| (b == cnvi.at).then_some((0xffff, 0xffff));
    let group = |i: usize| -> Vec<Member> {
        let hops = super::acs_path::path(funcs[i].at, &funcs).unwrap();
        super::acs_reach::group_by_hop(hops, acs)
            .into_iter()
            .map(|(m, hop)| Member { id: (m != xhci.at) as u64, at: m, hop, shared: false })
            .collect()
    };
    let gs = vec![group(0), group(1)];
    assert!(gs[1].is_empty() && gs[0].len() == 1, "the shape: only the xHCI reaches the other");
    let at = [xhci.at, cnvi.at];
    assert!(refused(&gs, &at, (0, false), (1, false)), "xHCI after CNVi");
    assert!(refused(&gs, &at, (1, false), (0, false)), "CNVi after xHCI: the hole K11b left");
}

#[test]
fn a_peer_carrying_the_devices_requester_id_is_named() {
    let line = peer_alias_line((3, 0, 0), "driver.nvme0");
    assert_eq!(line, "refused: 03:00.0, which driver.nvme0 holds, carries its requester id");
}
