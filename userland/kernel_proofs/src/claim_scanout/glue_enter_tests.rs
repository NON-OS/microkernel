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

//! The entry and the count the model's bootfb stands for, read (L79): in
//! enter.rs bootfb's claim is refused once surrendered, in the entry's own
//! hold, after the wipe record's answer and before the push; in lookup.rs an
//! unmap counts itself inside a claim that is Ready, Leaving or Exiting.

use super::glue_text::{body, in_order, source};

const ENTER: &str = "hardware/broker/claim/enter.rs";
const LOOKUP: &str = "hardware/broker/claim/lookup.rs";

#[test]
fn bootfb_is_refused_once_surrendered_after_the_wipe_answer_before_the_push() {
    let f = body(ENTER, "enter");
    in_order(
        &f,
        &[
            "let mut claims = claims();",
            "if let Some(c) = claims.iter().find(|c| c.device_id == device_id) \
         { return Err(over(c.stage)); }",
            "match fresh.map(|f| wipe::admits(device_id, f.by, capped)) {",
            "Some(Admit::Refused) => { drop(claims);",
        ],
    );
    // Right after the match closes, right before the push, in the one hold.
    let refusal = "return Err(ClaimError::Refused); } } \
        if crate::hardware::broker::bootfb::is_boot_framebuffer(device_id) && surrendered() \
        { return Err(ClaimError::Refused); } \
        let epoch = next_epoch(); claims.push(Claim::making(pid, device_id, epoch)); Ok(epoch) }";
    assert!(f.ends_with(refusal), "the surrendered refusal moved: {f}");
    let using = "use crate::kernel_core::init::framebuffer::surrendered;";
    assert!(source(ENTER).contains(using), "another `surrendered` asked");
}

#[test]
fn an_unmap_counts_itself_inside_a_ready_leaving_or_exiting_claim() {
    assert_eq!(
        body(LOOKUP, "inside"),
        "{ let mut claims = claims(); \
         let held = |c: &&mut Claim| (c.pid, c.device_id, c.epoch) == (pid, device_id, epoch); \
         let c = claims.iter_mut().find(held)?; \
         if !matches!(c.stage, Stage::Ready | Stage::Leaving | Stage::Exiting) { return None; } \
         c.inflight = c.inflight.checked_add(1)?; Some(InUse(*c)) }"
    );
}
