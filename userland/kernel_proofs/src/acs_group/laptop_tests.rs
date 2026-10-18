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

//! K11b on the machines of A's letter: who reaches whom, and what is said.

use super::acs_path::path;
use super::acs_reach::group_by_hop;
use super::acs_refusal::{alias_line, refusal_line};
use super::acs_tree::{Bdf, Func};
use super::laptops::*;
use super::read_layer::acs;

pub(super) fn reach(parts: &[Part], at: Bdf) -> Vec<(Bdf, Bdf)> {
    let all: Vec<Func> = parts.iter().map(|p| p.func).collect();
    group_by_hop(path(at, &all).unwrap(), acs(parts))
}

fn alone(parts: &[Part], at: Bdf) -> bool {
    reach(parts, at).is_empty()
}

#[test]
fn wifi_on_1c0_and_nvme_on_1c4_both_go_ahead_on_spt_and_300_series() {
    for parts in [spt(), cnp()] {
        assert!(alone(&parts, WIFI) && alone(&parts, NVME));
    }
}

#[test]
fn cnvi_and_xhci_on_one_device_both_go_ahead() {
    let rciep = |f: u8, d: u16| part((0, 0x14, f), None, (0x8086, d), Some(9));
    let parts = vec![rciep(0, 0x9DED), rciep(2, 0x9DEF), rciep(3, 0x9DF0)];
    assert!(alone(&parts, (0, 0x14, 3)) && alone(&parts, (0, 0x14, 0)));
}

#[test]
fn a_pre_spt_pair_is_refused_and_the_line_names_member_hop_and_holder() {
    let parts = pre_spt();
    assert_eq!(reach(&parts, WIFI), [(RP4, RP0), (NVME, RP0)]);
    assert_eq!(reach(&parts, NVME), [(RP0, RP4), (WIFI, RP4)]);
    let line = refusal_line(NVME, RP0, "driver.nvme0");
    assert_eq!(
        line,
        "refused: 02:00.0 reaches it through 00:1c.0 without ACS; driver.nvme0 holds it"
    );
}

#[test]
fn an_alias_in_another_domain_is_named_with_its_holder() {
    let line = alias_line((3, 0, 0), "driver.xhci0");
    assert_eq!(line, "refused: its requester id 03:00.0 is in the domain driver.xhci0 holds");
}
