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

use crate::settings::schema::rows::{Block, BtLive, Live, Pill, Row};

pub const BLUETOOTH: &[Block] = &[
    Block {
        title: "Bluetooth",
        note: Some("Keyboards and mice over a USB adapter. S scans or stops."),
        pill: Pill::Bt,
        rows: &[
            Row::Live("Adapter", Live::Bt(BtLive::Controller)),
            Row::Live("Status", Live::Bt(BtLive::Status)),
            Row::Live("Scanning", Live::Bt(BtLive::Scanning)),
        ],
    },
    Block {
        title: "Pair a device",
        note: Some("P lets one new device pair. Type its digits and Enter, or Y or N."),
        pill: Pill::None,
        rows: &[
            Row::Live("Pairing", Live::Bt(BtLive::Window)),
            Row::Live("Asks", Live::Bt(BtLive::Asked)),
        ],
    },
    Block {
        title: "Paired devices",
        note: Some("Kept until this machine switches off. F forgets the highlighted one."),
        pill: Pill::None,
        rows: &[Row::BtDevices],
    },
];
