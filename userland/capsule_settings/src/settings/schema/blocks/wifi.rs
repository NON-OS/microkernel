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

use nonos_policy_proto::Field;

use crate::settings::schema::rows::{Block, Live, Pill, Row};

pub const WIFI: &[Block] = &[
    Block {
        title: "Wi-Fi",
        note: Some("Off leaves the network and stops every scan and join. W switches it."),
        pill: Pill::Radio,
        rows: &[Row::Field(Field::WifiRadio)],
    },
    Block {
        title: "Connection",
        note: Some("WPA2-Personal or open. WPA3 (SAE) cannot be joined."),
        pill: Pill::None,
        rows: &[Row::Live("Status", Live::WifiLink), Row::Live("Last join", Live::WifiJoin)],
    },
    Block {
        title: "Networks",
        note: Some("Enter scans. A click or C joins the highlighted one, D leaves."),
        pill: Pill::None,
        rows: &[Row::Networks],
    },
    Block {
        title: "Saved networks",
        note: Some("Sealed with the TPM key. R remembers joins, F forgets."),
        pill: Pill::None,
        rows: &[Row::Live("Remember networks I join", Live::WifiRemember), Row::Saved],
    },
];
