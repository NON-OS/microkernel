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

//! Passive scanning (Vol 4, Part E, 7.8.10 and 7.8.11) and the reports it
//! brings (Vol 4, Part E, 7.7.65.2).

use crate::sim::config::Config;
use crate::sim::events::adv_report;
use crate::sim::stack::stack;
use nonos_bt_hci::host::{Scan, DEFAULT_SCAN};
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::event::parse_event;
use nonos_bt_wire::hci::le_meta::split;
use nonos_bt_wire::hci::reports::adv_reports;
use nonos_bt_wire::WireError;

#[test]
fn passive_scan_parameters_and_enable_are_sent_as_laid_out() {
    let mut s = stack(Config::default());
    s.bring_up().unwrap();
    s.scan_passive(DEFAULT_SCAN).unwrap();
    s.scan_stop().unwrap();
    let cmds = &s.transport().pipes().commands;
    let tail: Vec<&[u8]> = cmds[cmds.len() - 3..].iter().map(|c| c.1.as_slice()).collect();
    assert_eq!(tail[0], [0x0b, 0x20, 7, 0x00, 0x60, 0x00, 0x30, 0x00, 0x00, 0x00]);
    assert_eq!(tail[1], [0x0c, 0x20, 2, 1, 1]);
    assert_eq!(tail[2], [0x0c, 0x20, 2, 0, 0]);
}

#[test]
fn scan_timing_outside_the_allowed_range_is_refused_unsent() {
    let mut s = stack(Config::default());
    let bad = [Scan { interval: 3, window: 3 }, Scan { interval: 0x60, window: 0x61 }];
    for scan in bad {
        assert_eq!(s.scan_passive(scan), Err(HciError::Wire(WireError::Reserved)));
    }
    assert!(s.transport().pipes().commands.is_empty());
}

#[test]
fn a_report_arrives_whole_through_next_event() {
    let mut s = stack(Config::default());
    s.bring_up().unwrap();
    let data = [2, 1, 6, 3, 3, 0x12, 0x18, 9, 9, b'k', b'e', b'y', b'b', b'o', b'a', b'r', b'd'];
    s.transport().pipes().send_event(&adv_report(7, &data));
    let event = s.next_event().unwrap();
    let event = event.or_else(|| s.next_event().unwrap()).unwrap();
    let (sub, rest) = split(parse_event(&event).unwrap().params).unwrap();
    assert_eq!(sub, 0x02);
    let reports = adv_reports(rest).unwrap();
    assert_eq!((reports[0].addr.0[0], reports[0].data, reports[0].rssi), (7, &data[..], -60));
}
