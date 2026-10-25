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

//! The barriers a device is promised to observe (libc's, DMB OSHST and
//! OSHLD on aarch64), held in the shipping text: on the host they and a
//! fence compile alike. A core fence orders only what other CPUs see, so
//! the driver keeps none in their place.

use super::source::{at, files, source};

#[test]
fn every_legacy_window_read_is_followed_by_the_read_barrier() {
    let read = source("regs/read.rs");
    for width in ["r8", "r16", "r32"] {
        let ordered = format!("m.{width}(offset);\n                nonos_libc::dma_rmb();");
        assert!(read.contains(&ordered), "{width} is not ordered before what follows it");
    }
}

#[test]
fn every_legacy_window_write_follows_the_write_barrier() {
    let write = source("regs/write.rs");
    for width in ["w8", "w16", "w32"] {
        let ordered = format!("nonos_libc::dma_wmb();\n                m.{width}(offset, value)");
        assert!(write.contains(&ordered), "{width} may pass the stores before it");
    }
}

#[test]
fn the_emptied_ring_is_ordered_before_the_modern_queue_is_handed_over() {
    let queue = source("setup/modern/queue.rs");
    assert!(at(&queue, "Queue::new(") < at(&queue, "nonos_libc::dma_wmb()"));
    assert!(at(&queue, "nonos_libc::dma_wmb()") < at(&queue, "setup_queue(c"));
}

#[test]
fn no_core_fence_stands_in_for_a_device_barrier() {
    for (path, text) in files() {
        let text = text.replace("compiler_fence(", "");
        assert!(!text.contains("fence(Ordering::"), "{} orders with a fence", path.display());
    }
}
