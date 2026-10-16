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

//! Orders the host cannot run, held in the shipping text: the barriers a
//! weakly ordered machine needs (the capsule ships for aarch64), the
//! transport choice, the modern handshake, and the server's reset.

use super::source::{at, source};

#[test]
fn the_index_is_published_after_a_release_fence_and_the_doorbell_after_another() {
    let post = source("queue/post.rs");
    assert!(at(&post, "fence(Ordering::Release)") < at(&post, "avail + AVAIL_IDX"));
    assert!(at(&post, "write_chain(") < at(&post, "fence(Ordering::Release)"));
    let access = source("transport/access.rs");
    assert!(at(&access, "fence(Ordering::SeqCst)") < at(&access, "regs.w16(LEG_QUEUE_NOTIFY"));
    let used = source("queue/used.rs");
    assert!(at(&used, "fence(Ordering::Acquire)") < at(&used, "used + USED_RING"));
}

#[test]
fn the_modern_transport_is_taken_whenever_the_function_presents_it() {
    let probe = source("transport/probe.rs");
    assert!(probe.contains("modern_caps(&dev.bars"));
    assert!(!probe.contains("wants_probe"), "a transitional function's capabilities go unread");
}

// The modern handshake (nonos_virtio's, proved in virtio_transport_proofs)
// is reset and read back by the clock before Bus Master, and takes FLUSH
// alone of the device's features; ACCESS_PLATFORM whenever offered.
#[test]
fn the_modern_device_is_reset_and_seen_reset_before_it_may_master_the_bus() {
    let live = source("setup/modern/live.rs");
    assert!(at(&live, "start_until(&common") < at(&live, "pci::MASTER"));
    assert!(at(&live, "pci::MASTER") < at(&live, "queue::setup("));
    assert!(live.contains("accept(&common, VIRTIO_BLK_F_FLUSH)"));
    assert!(live.contains("const VIRTIO_BLK_F_FLUSH: u64 = 1 << 9;"));
    let run = source("setup/modern/run.rs");
    assert!(at(&run, "if live.is_err()") < at(&run, "reset_until("));
}

#[test]
fn a_stalled_queue_is_reset_by_name_and_brought_up_again() {
    let runner = source("server/runner.rs");
    assert!(at(&runner, "dispatch(driver") < at(&runner, "settle(driver, &mut streak)"));
    // The log reads DEVICE_NEEDS_RESET (`say`) before the reset clears it.
    let recover = source("server/recover.rs");
    assert!(at(&recover, "say(driver, why") < at(&recover, "driver.quiesce()"));
    assert!(at(&recover, "driver.quiesce()") < at(&recover, "bring_up("));
}

#[test]
fn a_request_the_driver_does_not_serve_is_refused_before_the_device() {
    let dispatch = source("server/dispatch.rs");
    assert!(dispatch.contains("_ => {\n            let _ = reply_with_status(tx, req, E_INVAL);"));
}
