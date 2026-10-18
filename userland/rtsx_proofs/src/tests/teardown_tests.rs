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

//! C2: teardown quiesces the reader and gives it back before any frame it
//! can reach goes. The chip init fails on a command buffer that never
//! finishes, and the driver is dropped, as `start` drops it (main.rs).

use nonos_libc::{blocked, events, violations};

use super::rig::{knobs, taken};
use crate::init::init_hw;
use crate::model::{complete_late_at_teardown, Knobs};

fn failed_bring_up(late: bool) {
    let mut drv = taken();
    knobs(Knobs { stall_cmd: true, late, ..Knobs::default() });
    assert!(init_hw(&mut drv).is_err());
    drop(drv);
}

fn at(ev: &[String], prefix: &str) -> usize {
    ev.iter().position(|e| e.starts_with(prefix)).unwrap_or_else(|| panic!("no {prefix}: {ev:?}"))
}

#[test]
fn a_late_completion_at_teardown_lands_in_no_freed_frame() {
    complete_late_at_teardown();
    failed_bring_up(true);
    let ev = events();
    assert!(ev.iter().any(|e| e == "late completion"), "{ev:?}");
    assert_eq!(violations(), 0, "{ev:?}");
    assert!(blocked() > 0, "{ev:?}");
}

#[test]
fn the_release_is_the_last_call_and_no_frame_is_unmapped_before_it() {
    failed_bring_up(false);
    let ev = events();
    let release = at(&ev, "release");
    assert!(ev.iter().all(|e| !e.starts_with("dma_unmap")), "{ev:?}");
    let broker = ["claim", "mmio_map", "mmio_unmap", "pci", "dma_map", "dma_unmap", "release"];
    let last = ev.iter().rposition(|e| broker.iter().any(|b| e.starts_with(b)));
    assert_eq!(last, Some(release), "{ev:?}");
}

#[test]
fn the_engines_stop_and_bus_mastering_goes_off_before_the_release() {
    failed_bring_up(false);
    let ev = events();
    let release = at(&ev, "release");
    // The last: the claim writes the same word before bus mastering.
    let off = ev.iter().rposition(|e| e == "pci 0x4=0x0402").expect("Bus Master cleared");
    assert!(off < release, "{ev:?}");
    let last_stop = ev.iter().rposition(|e| e == "stop dma").expect("DMA stopped");
    assert!(ev.iter().rposition(|e| e == "stop cmd").is_some_and(|s| s < off), "{ev:?}");
    assert!(last_stop < off && at(&ev, "bier 0x0") < off, "{ev:?}");
}
