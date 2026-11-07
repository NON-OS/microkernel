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

use crate::settings::schema::rows::{Action, Block, Live, Pill, Row};
use nonos_policy_proto::Field;

pub const SECURITY: &[Block] = &[
    Block {
        title: "Lock screen",
        note: Some(
            "The passphrase is typed in the system's own prompt, never in this window, \
             and only a slow hash of it is kept. There is no way to recover a forgotten one.",
        ),
        pill: Pill::None,
        rows: &[
            Row::Action("Passphrase", Action::LockPassphrase),
            Row::Field(Field::AutoLockTimeout),
            Row::Field(Field::LockOnLid),
            Row::Field(Field::LockInhibitHolds),
        ],
    },
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
