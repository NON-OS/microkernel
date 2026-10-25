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

//! held.rs read: the takeover's steps and the hold's store are the model's,
//! all SeqCst, and `stopped()` loads its flags in the order of the model's
//! KERNEL gate, so the defect the model finds is the kernel's.

use super::gate::{Flag, Gate};
use super::glue_gate_tests::seqcst_only;
use super::text_items::flat_body;

const HELD: &str = "kernel_core/init/framebuffer/held.rs";

#[test]
fn a_takeover_counts_itself_then_waits_latches_gone_and_ends() {
    let stop = "TAKERS.fetch_add(1,SeqCst);super::drawing::wait_quiet();";
    assert_eq!(flat_body(HELD, "fn stop_drawing()"), stop);
    assert_eq!(flat_body(HELD, "fn set_gone()"), "GONE.store(true,SeqCst);");
    assert_eq!(flat_body(HELD, "fn end_takeover()"), "TAKERS.fetch_sub(1,SeqCst);");
    assert_eq!(flat_body(HELD, "fn set_held(held: bool)"), "HELD.store(held,SeqCst);");
}

#[test]
fn stopped_loads_all_three_in_the_models_order() {
    let body = flat_body(HELD, "fn stopped()");
    let reads: Vec<Flag> = body
        .split("||")
        .map(|term| match term {
            "HELD.load(SeqCst)" => Flag::Held,
            "GONE.load(SeqCst)" => Flag::Gone,
            "TAKERS.load(SeqCst)>0" => Flag::Takers,
            other => panic!("`stopped()` reads `{other}`"),
        })
        .collect();
    assert_eq!(reads, Gate::KERNEL.reads);
}

#[test]
fn the_flags_are_written_only_by_those_steps_and_only_with_seqcst() {
    let text = seqcst_only(HELD);
    for flag in ["staticHELD:AtomicBool", "staticGONE:AtomicBool", "staticTAKERS:AtomicU32"] {
        assert!(text.contains(flag), "`{flag}` gone");
    }
    assert_eq!(text.matches("TAKERS.fetch_").count(), 2, "TAKERS counted elsewhere");
    assert_eq!(text.matches("GONE.store(").count(), 1, "GONE stored elsewhere");
    assert_eq!(text.matches("HELD.store(").count(), 1, "HELD stored elsewhere");
}
