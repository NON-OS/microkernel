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

//! What the server may do once it runs, checked in the shipping text: it
//! runs only after the primary surface is shared, and the compositor draws
//! into those frames from then on. Releasing the device would have the
//! kernel free them under it, and nothing restarts a driver, so a stall is
//! recovered in place (`recover_tests`) and nothing in the server gives the
//! device back, unmaps a grant or leaves.

use std::path::{Path, PathBuf};

const MAIN: &str = include_str!("../../../capsule_driver_virtio_gpu/src/main.rs");
const LOOP: &str =
    include_str!("../../../capsule_driver_virtio_gpu/src/server/runner/loop_once.rs");

/// Every source file under `dir` of the driver, with its text.
pub fn sources(dir: &str) -> Vec<(PathBuf, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_driver_virtio_gpu/src");
    let (mut stack, mut out) = (vec![root.join(dir)], Vec::new());
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("driver source") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let text = std::fs::read_to_string(&path).expect("read");
                out.push((path, text));
            }
        }
    }
    out
}

#[test]
fn nothing_the_server_runs_gives_the_device_back_or_leaves() {
    for (path, text) in sources("server") {
        for call in ["mk_device_release", "quiesce(", "mk_exit", "mk_dma_unmap"] {
            assert!(!text.contains(call), "{} calls {call}", path.display());
        }
    }
    let dispatched = LOOP.find("dispatch(&driver").expect("dispatch");
    assert!(LOOP.find("settle(&mut driver)").is_some_and(|at| at > dispatched));
}

// Before the service is registered no client can have the surface: the
// compositor is handed it only over the service. A failed registration
// stops the device and gives it back before the capsule leaves.
#[test]
fn a_failed_registration_resets_and_releases_the_device_before_leaving() {
    let register = MAIN.find("mk_service_register(").expect("registration");
    let after = &MAIN[register..];
    let leave = after.find("mk_exit(").expect("an exit on failure");
    assert!(after[..leave].contains("driver.quiesce()"), "left with the device live");
}
