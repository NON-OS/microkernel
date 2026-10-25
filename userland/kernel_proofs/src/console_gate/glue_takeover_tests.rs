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

//! Where the kernel takes the takeover's and the hold's steps. A covering
//! claim stops the console before its admit, latches GONE only in the hold
//! that makes it ready, after every refusal, and ends its takeover after the
//! admit whatever it answered. bootfb's hold is set in the claims' lock and
//! waited out once that lock is let go. Nothing else takes those steps.

use super::text_items::{at, flat_body, naming};

const CLAIM: &str = "hardware/broker/claim/claim.rs";
const PENDING: &str = "hardware/broker/claim/pending.rs";
const SCANOUT: &str = "hardware/broker/claim/scanout.rs";
const HOLD: &str = "hardware/broker/bootfb/hold.rs";
const HELD: &str = "kernel_core/init/framebuffer/held.rs";

#[test]
fn a_covering_claim_stops_then_is_admitted_then_ends_its_takeover() {
    let c = flat_body(CLAIM, "pub fn claim(");
    let take = at(&c, "ifcovers{super::scanout::take(device_id);}");
    let admit = "letadmitted=super::pending::admit(pid,device_id,epoch,fresh.as_ref(),covers);";
    let end = "ifcovers{crate::kernel_core::init::framebuffer::end_takeover();}admitted.map(";
    assert!(take < at(&c, admit) && c.contains(&format!("{admit}{end}")));
    assert!(flat_body(SCANOUT, "pub(super) fn take(").starts_with("console::stop_drawing();"));
}

#[test]
fn gone_is_latched_only_once_every_refusal_is_past() {
    let a = flat_body(PENDING, "pub(super) fn admit(");
    let ready = at(&a, "claims[at].stage=Stage::Ready;");
    let latch = "ifcovers{crate::kernel_core::init::framebuffer::set_gone();}";
    let then = "crate::hardware::broker::bootfb::admitted(claims,device_id);Ok(())";
    assert!(a[ready..].starts_with(&format!("claims[at].stage=Stage::Ready;{latch}{then}")));
    assert!(at(&a, "super::scanout::admits(&claims,device_id,covers)?;") < ready);
    assert!(!a[ready..].contains("return") && !a[ready..].contains('?'), "a refusal after");
}

#[test]
fn the_hold_is_set_in_the_lock_and_waited_out_after_it() {
    let set = "ifis_boot_framebuffer(id){set_held(true);drop(claims);wait_quiet();say(true);}";
    assert_eq!(flat_body(HOLD, "fn admitted("), set);
    assert!(flat_body(HOLD, "fn released(").contains("iffb{set_held(false);}"));
}

#[test]
fn nothing_else_takes_the_gates_steps() {
    let only = |token, files: &[&str]| assert_eq!(naming(token), files, "`{token}`");
    only("stop_drawing()", &[SCANOUT, HELD]);
    only("set_gone()", &[PENDING, HELD]);
    only("end_takeover()", &[CLAIM, HELD]);
    only("set_held(", &[HOLD, HELD]);
    only("wait_quiet()", &[HOLD, "kernel_core/init/framebuffer/drawing.rs", HELD]);
}
