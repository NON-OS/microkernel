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

//! The claim steps the model's take-back and unmap stand on, read whole
//! (L79), though they came before it and no proof read them: a release's
//! retire waits out every path inside the epoch it retired before anything
//! is drained (the model's INFLIGHT step), and the last path out of a claim
//! being released takes its entry away (the model's `edit`). Undone, the
//! first lets a map record its grant after the drain, past the repoint, and
//! the second leaves the entry for ever, the takeover waiting on it.

use super::super::glue_text::body;

const LEAVE: &str = "hardware/broker/claim/leave.rs";
const LOOKUP: &str = "hardware/broker/claim/lookup.rs";

#[test]
fn a_retire_waits_out_every_path_inside_its_epoch() {
    let want = [
        "{ let epoch = { let mut claims = claims();",
        "let c = claims.iter_mut().find(|c| c.device_id == device_id);",
        "let c = c.ok_or(ClaimError::NotClaimed)?;",
        "if c.pid != pid { return Err(ClaimError::NotHolder); } c.stage = retired(c.stage)?;",
        "c.leavers = c.leavers.checked_add(1).ok_or(ClaimError::NotClaimed)?; c.epoch };",
        "let inside = |c: &Claim| c.device_id == device_id && c.epoch == epoch && c.inflight > 0;",
        "while claims().iter().any(inside) { crate::smp::serve_shootdowns();",
        "core::hint::spin_loop(); } Ok(epoch) }",
    ];
    assert_eq!(body(LEAVE, "retire"), want.join(" "));
}

#[test]
fn the_last_path_out_of_a_going_claim_takes_its_entry_away() {
    let want = [
        "{ let key = (self.0.pid, self.0.device_id, self.0.epoch); let mut claims = claims();",
        "let at = claims.iter().position(|c| (c.pid, c.device_id, c.epoch) == key);",
        "let Some(at) = at else { return }; let c = &mut claims[at];",
        "c.inflight = c.inflight.saturating_sub(1);",
        "if gone(c.stage, c.leavers, c.inflight) { super::gone::remove(claims, at); } }",
    ];
    assert_eq!(body(LOOKUP, "drop"), want.join(" "));
}
