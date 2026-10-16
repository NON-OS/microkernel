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

//! The extended scanner filter policy (Vol 4, Part E, 7.8.10; Vol 6,
//! Part B, 4.3.3) is used only by a controller whose LE features have
//! Extended Scanner Filter Policies (bit 7, Vol 6, Part B, 4.6), and only
//! such a controller is asked for LE Directed Advertising Reports (bit 10
//! of LE Set Event Mask, 7.8.1).

use crate::sim::config::Config;
use crate::sim::stack::stack;
use nonos_bt_hci::host::DEFAULT_SCAN;
use nonos_bt_wire::hci::opcode as op;

/// The LE event mask and the scan's filter policy the host sent.
fn sent(le_features: u64) -> (Vec<u8>, u8, u64) {
    let mut s = stack(Config { le_features, ..Config::default() });
    let c = s.bring_up().unwrap();
    s.scan_passive(DEFAULT_SCAN).unwrap();
    let cmds = &s.transport().pipes().commands;
    let find = |o: u16| cmds.iter().find(|c| c.1[..2] == o.to_le_bytes()).unwrap().1.clone();
    let mask = find(op::LE_SET_EVENT_MASK.0)[3..].to_vec();
    (mask, find(op::LE_SET_SCAN_PARAMETERS.0)[9], c.le_features)
}

#[test]
fn a_controller_with_the_feature_scans_with_policy_two_and_reports_directed() {
    let (mask, policy, features) = sent(0x81);
    assert_eq!(mask, [0x1f, 0x04, 0, 0, 0, 0, 0, 0]);
    assert_eq!((policy, features), (0x02, 0x81));
}

#[test]
fn a_controller_without_it_keeps_policy_zero_and_the_plain_mask() {
    for features in [0, 1 << 6, 1 << 8, !(1 << 7)] {
        let (mask, policy, _) = sent(features);
        assert_eq!((mask, policy), (vec![0x1f, 0, 0, 0, 0, 0, 0, 0], 0x00), "{features:#x}");
    }
}

#[test]
fn features_are_read_little_endian_and_a_short_answer_stops_bring_up() {
    assert_eq!(sent(0x0102_0304_0506_0788).2, 0x0102_0304_0506_0788);
    let short = vec![(
        op::LE_READ_LOCAL_FEATURES.0,
        vec![crate::sim::events::complete(1, 0x2003, 0, &[0x80])],
    )];
    assert!(stack(Config::scripted(short)).bring_up().is_err());
}
