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

//! The words the rows read, and what a request the page made did.

use super::fixture::{answered, list, two_bonds};
use crate::bluetooth::panel::{
    addr_text, controller, level_of, made, scanning, status, Panel, Reach,
};
use crate::settings::schema::rows::Tone;
use nonos_bt_proto::ops::*;
use nonos_bt_proto::{Health, Link};

#[test]
fn the_status_row_reads_the_link() {
    let ready = |flags| status(&answered(READY, flags)).0;
    assert_eq!(ready(Link::KEYBOARD | Link::MOUSE), "Keyboard and mouse connected");
    assert_eq!(ready(Link::KEYBOARD), "Keyboard connected");
    assert_eq!(ready(Link::MOUSE), "Mouse connected");
    assert_eq!(status(&answered(READY, Link::MOUSE)).1, Tone::Ok);
    assert_eq!(status(&answered(SECURING, 0)).0, "Securing the link");
    assert_eq!(status(&answered(IDLE, 0)).0, "Not looking for devices");
    let mut gone = answered(READY, Link::KEYBOARD);
    gone.lost(Reach::NoService);
    assert_eq!(status(&gone), ("Bluetooth is not running", Tone::Warn));
    assert_eq!(scanning(&gone).0, "--");
    assert_eq!(status(&Panel::new()).0, "--");
}

#[test]
fn the_adapter_row_reads_health() {
    let mut p = answered(IDLE, 0);
    assert_eq!(controller(&p).0, "No USB adapter found", "no HEALTH yet");
    assert!(p.took_health(&Health { version: 1, controller: true }.write()));
    assert_eq!(controller(&p), ("USB adapter ready", Tone::Ok));
    assert!(p.took_health(&Health { version: 2, controller: true }.write()));
    assert_eq!(controller(&p), ("Speaks another NBTC", Tone::Warn));
    assert_eq!(scanning(&answered(SCANNING, Link::SCANNING)).0, "On");
    assert_eq!(scanning(&p).0, "Off");
}

#[test]
fn a_device_reads_its_address_and_its_lowest_level() {
    let mut p = answered(READY, Link::MOUSE);
    two_bonds(&mut p);
    assert_eq!(&addr_text(&p.bonds[0].addr), b"C6:55:44:33:22:11");
    assert_ne!(made(&p.bonds[0]), made(&p.bonds[1]), "authenticated or not");
    assert!(p.took_levels(&list(&[vec![1, 0, 80], vec![1, 1, 35], vec![0xFF, 0, 5]])));
    assert_eq!((level_of(&p, 1), level_of(&p, 0)), (Some(35), None));
}
