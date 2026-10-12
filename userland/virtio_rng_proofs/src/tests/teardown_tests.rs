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

//! Teardown and recovery, which call the kernel and so cannot run on the
//! host, checked in the shipping text (plan/50 C1, C2).

const SRC: &str = "../../../capsule_driver_virtio_rng/src/";
const DRIVER: &str = include_str!("../../../capsule_driver_virtio_rng/src/setup/driver.rs");
const RUNNER: &str = include_str!("../../../capsule_driver_virtio_rng/src/server/runner.rs");
const RECOVER: &str = include_str!("../../../capsule_driver_virtio_rng/src/server/recover.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("`{needle}` not found"))
}

// Teardown stops the queue, then hands the device back: the broker clears
// bus mastering and only then frees the grants. Nothing is unmapped first.
#[test]
fn teardown_resets_the_device_and_never_unmaps_dma_itself() {
    assert!(at(DRIVER, "self.transport.reset()") < at(DRIVER, "mk_device_release("));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests").join(SRC);
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("driver source") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let text = std::fs::read_to_string(&path).expect("read");
                assert!(!text.contains("mk_dma_unmap"), "{} unmaps DMA", path.display());
            }
        }
    }
}

// A request that stalled ends in a reset and a fresh bring-up, by name.
#[test]
fn a_stalled_queue_is_reset_and_brought_up_again() {
    assert!(at(RUNNER, "driver.queue.stalled()") < at(RUNNER, "recover(driver)"));
    assert!(at(RECOVER, "driver.quiesce()") < at(RECOVER, "bring_up("));
}
