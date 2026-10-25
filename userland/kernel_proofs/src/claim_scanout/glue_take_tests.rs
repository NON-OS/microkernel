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

//! scanout.rs, the takeover the model's G1 and G2 stand for, read (L79):
//! the console stopped first, then the loop on `next` over one read of
//! bootfb's claim at a time; a take-back's retire, page move and release in
//! that order; and the admit's check on the table its caller holds.

use super::glue_text::{body, count, in_order};

const SCANOUT: &str = "hardware/broker/claim/scanout.rs";

#[test]
fn the_console_stops_first_then_next_decides_each_turn_of_the_loop() {
    let f = body(SCANOUT, "take");
    in_order(
        &f,
        &[
            "{ console::stop_drawing();",
            "let (Some(fb), Some(sink)) = (bootfb::device(), bootfb::sink()) else { return; };",
            "loop { let seen = peek(fb); match (next(seen.map(|s| s.1)), seen) { \
         (Next::TakeBack, Some((pid, _))) => take_back(by, pid, fb, sink), \
         (Next::Done, _) | (Next::TakeBack, None) => return, \
         (Next::Wait, _) => { crate::smp::serve_shootdowns(); core::hint::spin_loop(); } } } }",
        ],
    );
    assert!(f.starts_with("{ console::stop_drawing();"), "something before the stop");
    let peek = "{ claims().iter().find(|c| c.device_id == fb).map(|c| (c.pid, c.stage)) }";
    assert_eq!(body(SCANOUT, "peek"), peek);
}

#[test]
fn a_take_back_retires_then_moves_the_pages_then_releases() {
    let f = body(SCANOUT, "take_back");
    in_order(
        &f,
        &[
            "{ let Ok(epoch) = super::leave::retire(pid, fb) else { return; };",
            "let pages = mmio::take_back(pid, fb, epoch, sink);",
            "let _ = super::release::release(pid, fb, epoch);",
        ],
    );
    assert_eq!(count(&f, "release("), 1, "a second release");
}

#[test]
fn the_admit_check_reads_bootfbs_stage_from_the_table_it_is_handed() {
    let f = body(SCANOUT, "admits");
    let want = "{ let fb = bootfb::device(); \
        let stage = claims.iter().find(|c| Some(c.device_id) == fb).map(|c| c.stage); \
        at_admit(fb == Some(device_id), console::surrendered(), covers, stage) }";
    assert_eq!(f, want);
}
