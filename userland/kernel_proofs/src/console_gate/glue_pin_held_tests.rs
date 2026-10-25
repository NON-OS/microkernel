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

//! held.rs pinned whole, so nothing the model leaves out can be added to
//! it: no flag that starts set, no answer read off the wrong flag, no store
//! the model does not take. `stopped()` and `taken()` are built from the
//! model's KERNEL loads, so the kernel and the model change together.

use super::gate::{Flag, Gate};
use super::text::code;
use super::text_items::flat;

const FB: &str = "kernel_core/init/framebuffer/";

fn load(f: &Flag) -> &'static str {
    match f {
        Flag::Held => "HELD.load(SeqCst)",
        Flag::Gone => "GONE.load(SeqCst)",
        Flag::Takers => "TAKERS.load(SeqCst)>0",
    }
}

#[test]
fn held_rs_is_the_models_three_flags_and_nothing_else() {
    let reads = Gate::KERNEL.reads;
    let stopped: Vec<&str> = reads.iter().map(load).collect();
    let taken: Vec<&str> = reads.iter().filter(|f| **f != Flag::Held).map(load).collect();
    let stopped = format!("pub(super)fnstopped()->bool{{{}}}", stopped.join("||"));
    let taken = format!("pub(crate)fntaken()->bool{{{}}}", taken.join("||"));
    let want = [
        "usecore::sync::atomic::{AtomicBool,AtomicU32,Ordering::SeqCst};",
        "staticHELD:AtomicBool=AtomicBool::new(false);",
        "staticGONE:AtomicBool=AtomicBool::new(false);",
        "staticTAKERS:AtomicU32=AtomicU32::new(0);",
        "pub(crate)fnset_held(held:bool){HELD.store(held,SeqCst);}",
        &stopped,
        "pub(crate)fnstop_drawing(){TAKERS.fetch_add(1,SeqCst);",
        "super::drawing::wait_quiet();}",
        "pub(crate)fnset_gone(){GONE.store(true,SeqCst);}",
        "pub(crate)fnend_takeover(){TAKERS.fetch_sub(1,SeqCst);}",
        "pub(crate)fnsurrendered()->bool{GONE.load(SeqCst)}",
        &taken,
    ];
    assert_eq!(flat(&code(&format!("{FB}held.rs"))), want.concat());
}
