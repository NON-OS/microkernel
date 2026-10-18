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

//! The Bluetooth page: where it sits, and what its cards hold. It sets no
//! policy field, so the cursor stops nowhere on it and no row is stored.

use crate::settings::schema::rows::{BtLive, Live, Pill, Row};
use crate::settings::schema::{blocks_for, field_count};
use crate::settings::section::{Section, SECTIONS};

#[test]
fn bluetooth_follows_wi_fi() {
    let at = SECTIONS.iter().position(|s| *s == Section::Bluetooth).unwrap();
    assert!(SECTIONS[at - 1] == Section::Wifi);
    assert_eq!(field_count(Section::Bluetooth), 0);
}

#[test]
fn its_cards_read_the_service_and_list_the_devices() {
    let blocks = blocks_for(Section::Bluetooth);
    assert!(matches!(blocks[0].pill, Pill::Bt));
    let rows: Vec<Row> = blocks.iter().flat_map(|b| b.rows.iter().copied()).collect();
    let lives: Vec<BtLive> = rows
        .iter()
        .filter_map(|r| match r {
            Row::Live(_, Live::Bt(w)) => Some(*w),
            _ => None,
        })
        .collect();
    let every =
        [BtLive::Controller, BtLive::Status, BtLive::Scanning, BtLive::Window, BtLive::Asked];
    assert!(lives == every, "each live row once, in the page's order");
    assert_eq!(rows.iter().filter(|r| matches!(r, Row::BtDevices)).count(), 1);
    assert_eq!(rows.len(), every.len() + 1, "nothing else: no field is stored");
}
