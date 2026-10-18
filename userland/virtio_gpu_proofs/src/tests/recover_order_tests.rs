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

//! What the server may do once it runs, checked in the shipping text: the
//! compositor draws into its primaries and every output shows a surface
//! whose pages the driver holds. Releasing the device would have the kernel
//! free them under the host, and nothing restarts a driver, so a stall is
//! recovered in place (`gpu::recover_tests`) and nothing in the server gives
//! the device back, unmaps a grant or leaves; but for a DGON naming the
//! device, by which time the kernel has revoked every grant (`gone.rs`).

use std::path::{Path, PathBuf};

const MAIN: &str = include_str!("../../../capsule_driver_virtio_gpu/src/main.rs");
const LOOP: &str = include_str!("../../../capsule_driver_virtio_gpu/src/server/run.rs");

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
        let gone = path.ends_with("gone.rs");
        for call in ["mk_device_release", "quiesce(", "mk_exit", "mk_dma_unmap"] {
            let allowed = gone && call == "mk_exit";
            assert!(allowed || !text.contains(call), "{} calls {call}", path.display());
        }
    }
    // Every turn settles first, so a stall a request's commands hit is
    // recovered before the next frame is read.
    let settled = LOOP.find("settle(&mut driver)").expect("settle");
    assert!(settled < LOOP.find("mk_ipc_recv_from(").expect("the receive"));
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
