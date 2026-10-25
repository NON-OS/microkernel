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

//! The claim path the model's covering claim stands for, read (L79): in
//! claim.rs, covers read before the entry, the takeover inside `if covers`
//! after the Bus Master stop and before the admit, its end after the admit
//! whatever it answered, and the withdraw on a refusal; in pending.rs, the
//! admit's checks before anything is written, and GONE latched only there.

use super::glue_text::{at, body, count, in_order};

const CLAIM: &str = "hardware/broker/claim/claim.rs";
const PENDING: &str = "hardware/broker/claim/pending.rs";

#[test]
fn the_takeover_runs_between_the_last_refusal_and_the_admit() {
    let f = body(CLAIM, "claim");
    in_order(
        &f,
        &[
            "let covers = crate::hardware::broker::bootfb::covers(device_id);",
            "let epoch = super::enter::enter(pid, device_id, fresh.as_ref())?;",
            "if !hold::untranslated(device_id) { super::master_off::bus_master_off(device_id); }",
            "if covers { super::scanout::take(device_id); }",
            "let admitted = super::pending::admit(pid, device_id, epoch, fresh.as_ref(), covers);",
            "if covers { crate::kernel_core::init::framebuffer::end_takeover(); }",
            "admitted.map(|()| epoch)\
         .inspect_err(|_| super::pending::withdraw(pid, device_id, epoch)) }",
        ],
    );
    assert_eq!(count(&f, "scanout::take("), 1, "a second takeover");
    assert_eq!(count(&f, "end_takeover()"), 1, "a second end");
    // Nothing between the take and the end may leave the call early.
    let span = &f[at(&f, "scanout::take(")..at(&f, "end_takeover()")];
    assert!(!span.contains("return") && !span.contains('?'), "a way out: {span}");
}

#[test]
fn the_admit_asks_before_it_writes_and_latches_gone_only_for_a_cover() {
    let f = body(PENDING, "admit");
    assert!(f.starts_with("{ let mut claims = claims();"), "the hold not taken first");
    in_order(
        &f,
        &[
            "let Some(at) = at.filter(|&i| claims[i].stage == Stage::Making) else \
         { return Err(ClaimError::NotClaimed); };",
            "super::scanout::admits(&claims, device_id, covers)?;",
            "if f.is_some_and(|f| !super::wipe::held(device_id, epoch, f)) \
         { return Err(ClaimError::NotClaimed); }",
            "claims[at].stage = Stage::Ready;",
            "if covers { crate::kernel_core::init::framebuffer::set_gone(); }",
            "crate::hardware::broker::bootfb::admitted(claims, device_id); Ok(()) }",
        ],
    );
    assert_eq!(count(&f, "set_gone()"), 1, "GONE latched twice");
    assert!(!f.contains("drop(claims)"), "the hold let go in the admit's middle");
}
