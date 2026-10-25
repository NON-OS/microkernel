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

//! MkToolRun's bound: a tool holds no more than its caller, unless the caller
//! is a launcher; and the run path checks it before every spawn it makes.

use crate::stand_in::{said, world};
use crate::tool_bound::check;

const APP: u64 = 0x1 | 0x8 | 0x10;
const IPC_ONLY: u64 = 0x1 | 0x8;
const NET: u64 = 1 << 2;
const ADMIN: u64 = 1 << 9;
const BROKER: u64 = 1 << 23;
const WINDOW: u64 = 1 << 24;
/* tool.install's set: CoreExec IPC Memory Crypto FileSystem Admin DeviceEnum StoreWrite. */
const INSTALL: u64 = 0x0400_8279;

fn manifest(required: u64, optional: u64) -> Vec<u8> {
    [required.to_be_bytes(), optional.to_be_bytes()].concat()
}

#[test]
fn a_caller_without_the_bits_is_refused_and_it_is_said() {
    world(Some(7), &[(7, APP)]);
    assert_eq!(check(&manifest(APP | NET, 0), 0, b"tool.x"), Err(-1));
    assert_eq!(said(), "[CAP-DENY] tool: caller pid=7 tool=tool.x missing caps 0x4\n");
    world(Some(7), &[(7, APP)]);
    assert_eq!(check(&manifest(APP, NET), 0, b"tool.x"), Ok(()), "an optional bit not asked");
    assert_eq!(check(&manifest(APP, NET), NET, b"tool.x"), Err(-1), "an optional bit asked");
}

#[test]
fn an_ipc_only_capsule_that_starts_tool_install_is_refused() {
    world(Some(9), &[(9, IPC_ONLY)]);
    assert_eq!(check(&manifest(INSTALL, 0x100), INSTALL, b"tool.install"), Err(-1));
    assert!(said().contains("tool=tool.install missing caps 0x4008260"));
    world(Some(9), &[(9, IPC_ONLY | BROKER)]);
    assert_eq!(check(&manifest(INSTALL, 0), INSTALL, b"tool.install"), Err(-1), "a broker is no launcher");
}

#[test]
fn a_launcher_may_run_a_tool_beyond_itself() {
    for launcher in [WINDOW, ADMIN] {
        world(Some(4), &[(4, IPC_ONLY | launcher)]);
        assert_eq!(check(&manifest(INSTALL, 0), INSTALL, b"tool.install"), Ok(()));
        assert_eq!(said(), "");
    }
}

#[test]
fn no_manifest_is_left_to_the_spawn_and_no_caller_is_refused() {
    world(Some(4), &[(4, 0)]);
    assert_eq!(check(&[], u64::MAX, b"tool.linux"), Ok(()));
    world(None, &[(0, u64::MAX)]);
    assert_eq!(check(&manifest(APP, 0), 0, b"tool.x"), Err(-1));
    world(Some(4), &[(4, APP)]);
    assert_eq!(check(&[1, 2, 3], 0, b"tool.x"), Err(-1), "a manifest that does not decode");
}
