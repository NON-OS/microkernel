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

use crate::settings::schema::rows::{Block, Pill, Row};

pub const SECURITY: &[Block] = &[
    Block {
        title: "Keys",
        note: Some("Made on this machine by the setup wizard."),
        pill: Pill::None,
        rows: &[Row::Field(Field::SystemKeysGenerated)],
    },
    Block {
        title: "Kernel protections",
        note: Some("SMEP, SMAP, UMIP, NX and WP are set at boot when the CPU has them."),
        pill: Pill::None,
        rows: &[],
    },
];
