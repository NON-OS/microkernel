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

//! The three roles nsh ships, the reviewed table plan step P6 asks for. A
//! fleet's role table writes them as they are here. No ceiling names any of
//! the forbidden nine, nor `TimeSet` (the grant checks trust the host clock),
//! `InputConsumer` (every keystroke), `SpawnBroker`, `ForeignExec` or any
//! driver capability; until kernel change K2, nor `Crypto` or
//! `RegisterService`. Each list is in ascending byte order, as written.

pub struct RoleSpec {
    pub name: &'static str,
    pub record_digest: bool,
    pub approvals: u8,
    pub max_session_ms: u64,
    pub ceiling: &'static [&'static str],
    pub escalation: &'static [&'static str],
}

const HOUR: u64 = 3_600_000;

/// In ascending name order, as a table lists its roles.
#[rustfmt::skip]
pub const SHIPPED: [RoleSpec; 3] = [
    // Reads the machine's state and its logs; never escalates.
    RoleSpec {
        name: "auditor",
        record_digest: false,
        approvals: 2,
        max_session_ms: 8 * HOUR,
        ceiling: &["AttestRead", "CoreExec", "FileSystem", "IPC", "Memory"],
        escalation: &[],
    },
    // Puts data on the machine. Installing a capsule (AppInstall makes an
    // outbound fetch) and the network come only under an approved escalation.
    RoleSpec {
        name: "deploy",
        record_digest: false,
        approvals: 1,
        max_session_ms: HOUR,
        ceiling: &[
            "CoreExec", "FileSystem", "IPC", "Memory", "StoreWrite", "StreamImport",
        ],
        escalation: &[
            "AppInstall", "CoreExec", "FileSystem", "IPC", "Memory", "Network",
            "ProcessControl", "StoreWrite", "StreamImport",
        ],
    },
    // Runs the machine day to day; more under an approved escalation.
    RoleSpec {
        name: "operator",
        record_digest: false,
        approvals: 1,
        max_session_ms: 8 * HOUR,
        ceiling: &["AttestRead", "CoreExec", "FileSystem", "IPC", "Memory", "ProcessControl"],
        escalation: &[
            "AttestRead", "CoreExec", "FileSystem", "IPC", "Memory", "Network",
            "ProcessControl", "StoreWrite",
        ],
    },
];
