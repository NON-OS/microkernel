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

//! The admit and the claim's last steps read whole (L79). The first checks
//! found each step after the one before, which let a line slip in between:
//! Ready written ahead of the scanout check passed them, and the claim's
//! own refusal then withdraws an entry a sibling thread may have mapped.

use super::super::glue_text::body;

const CLAIM: &str = "hardware/broker/claim/claim.rs";
const PENDING: &str = "hardware/broker/claim/pending.rs";

#[test]
fn the_admit_asks_everything_before_it_writes_anything() {
    let want = [
        "{ let mut claims = claims(); let at = claims.iter().position(is(pid, device_id, epoch));",
        "let Some(at) = at.filter(|&i| claims[i].stage == Stage::Making) else",
        "{ return Err(ClaimError::NotClaimed); };",
        "super::scanout::admits(&claims, device_id, covers)?;",
        "if f.is_some_and(|f| !super::wipe::held(device_id, epoch, f))",
        "{ return Err(ClaimError::NotClaimed); } claims[at].stage = Stage::Ready;",
        "if covers { crate::kernel_core::init::framebuffer::set_gone(); }",
        "crate::hardware::broker::bootfb::admitted(claims, device_id); Ok(()) }",
    ];
    assert_eq!(body(PENDING, "admit"), want.join(" "));
}

#[test]
fn the_claim_ends_with_the_master_stop_the_takeover_the_admit_and_its_end() {
    let tail = [
        "super::no_snoop::snoop_every_request(device_id);",
        "if !hold::untranslated(device_id) { super::master_off::bus_master_off(device_id); }",
        "if covers { super::scanout::take(device_id); }",
        "let admitted = super::pending::admit(pid, device_id, epoch, fresh.as_ref(), covers);",
        "if covers { crate::kernel_core::init::framebuffer::end_takeover(); }",
        "admitted.map(|()| epoch)\
         .inspect_err(|_| super::pending::withdraw(pid, device_id, epoch)) }",
    ];
    let f = body(CLAIM, "claim");
    assert!(f.ends_with(&tail.join(" ")), "{f}");
}
