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

//! Any run of guest maps, pins, unpins, reprotects, unmaps and exits and the
//! compositor's attaches and releases, through the kernel's own pin, register,
//! unmap and rights rules mounted unchanged, held to the kernel_proofs pin
//! model (surface_pin/model.rs). After every step no view reads a freed frame
//! or another owner's, every view is read-only, and every frame the registry
//! keeps is a live surface's or listed to free; once every view lets go and
//! every guest exits, no frame is left held. An unmap meeting part of a pin
//! under a view is among the steps, as a guest's supervisor unmaps a span a
//! MiB at a time.

#![no_main]

use libfuzzer_sys::fuzz_target;

// Its shape and reader checks are peer_pin's, not this run's.
#[allow(dead_code)]
#[path = "../../../../src/kernel_core/surface_registry/pin/pin_rule.rs"]
mod pin_rule;
#[path = "../../../../src/kernel_core/surface_registry/pin/register_rule.rs"]
mod register_rule;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_attach.rs"]
mod rights_attach;
// Its write share and pid revoke are surface_pin's own tests', not this run's.
#[allow(dead_code)]
#[path = "../../../../src/kernel_core/surface_registry/share/rights_rule.rs"]
mod rights_rule;
#[path = "../../../../src/kernel_core/surface_registry/pin/unmap_rule.rs"]
mod unmap_rule;
#[path = "../../src/surface_pin/model.rs"]
mod model;
use model::{World, PAGES};

fuzz_target!(|data: &[u8]| {
    let mut w = World::new();
    let mut handles: Vec<u64> = Vec::new();
    for op in data.chunks_exact(3).take(256) {
        let g = 1 + u32::from(op[1] % 3);
        let p = usize::from(op[1] >> 2) % PAGES;
        let n = 1 + usize::from(op[2] % 4);
        let to = (p + n).min(PAGES);
        let pick = handles.get(usize::from(op[2] >> 2) % handles.len().max(1)).copied();
        match op[0] % 9 {
            0 | 1 => (p..to).for_each(|q| w.map(g, q)),
            2 => handles.extend(w.pin(g, p, to - p)),
            3 => pick.into_iter().for_each(|h| w.attach(h)),
            4 => pick.into_iter().for_each(|h| w.unpin(h)),
            5 => pick.into_iter().for_each(|h| w.let_go(h)),
            6 => w.unmap(g, p, to),
            7 => w.protect_exec(g, p),
            _ => w.exit(g),
        }
        w.check();
    }
    handles.iter().for_each(|&h| w.let_go(h));
    (1..=3).for_each(|g| w.exit(g));
    w.check();
    assert_eq!(w.held(), 0, "a frame was never given back");
});
