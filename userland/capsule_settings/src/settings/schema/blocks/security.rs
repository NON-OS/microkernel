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

use crate::settings::schema::rows::{Block, Live, Pill, Row};

pub const SECURITY: &[Block] = &[
    // No "keys generated" switch: nothing sets that field, so it read No on
    // every machine. The row asks the TPM itself when the page opens.
    Block {
        title: "Machine key",
        note: Some(
            "Asked of the TPM when this page opens. Saved Wi-Fi passphrases are sealed with it.",
        ),
        pill: Pill::None,
        rows: &[Row::Live("Machine key", Live::MachineKey)],
    },
    Block {
        title: "Kernel protections",
        note: Some("SMEP, SMAP, UMIP, NX and WP are set at boot when the CPU has them."),
        pill: Pill::None,
        rows: &[],
    },
];
