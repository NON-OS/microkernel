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


//! The orders L80 rests on, read where the models cannot run them, each
//! function pinned whole (whole). A claim's kept bound is a closure made in
//! the claims hold, which the wipes hold calls only once the record is read,
//! and whose answer the rule is given (entry_pin_tests has the claim that
//! asks it, finish_pin_tests the end's side, glue_libc_tests libc's).

use super::super::model_enter::KEEP_CAP;
use super::code::flat;
use super::whole::{num, whole};

const ASK: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/ask.rs");
const ENTER: &str = include_str!("../../../../../src/hardware/broker/claim/enter.rs");

const ADMITS: [&str; 5] = [
    "pub(in crate::hardware::broker) fn admits( device_id: u64, by: Option<Name>,",
    "over_cap: impl FnOnce() -> bool, ) -> Admit { let all = WIPES.lock();",
    "let record = all.iter().find(|w| w.device_id == device_id);",
    "let (now, capped) = (crate::time::now_ns(), over_cap());",
    "rule(record.map(|w| w.phase), same(record.and_then(|w| w.by), by), now, capped) }",
];

const ENTERS: [&str; 14] = [
    "pub(super) fn enter(pid: u32, device_id: u64, fresh: Option<&Fresh>)",
    "-> Result<u64, ClaimError> { wipe::step_all(true); let mut claims = claims();",
    "if let Some(c) = claims.iter().find(|c| c.device_id == device_id) {",
    "return Err(over(c.stage)); }",
    "let capped = || crate::hardware::broker::dma::kept_bytes(device_id) >= KEEP_CAP;",
    "match fresh.map(|f| wipe::admits(device_id, f.by, capped)) { None | Some(Admit::Go) => {}",
    "Some(Admit::Rearm) => { wipe::rearm(device_id); return Err(ClaimError::Resetting); }",
    "Some(Admit::Resetting) => return Err(ClaimError::Resetting),",
    r#"Some(Admit::Refused) => { drop(claims); crate::log::info!( "[BROKER] pid={} refused"#,
    r#"device {}: kept for its last holder's driver service, or \ past its kept frames'"#,
    r#"bound", pid, device_id ); return Err(ClaimError::Refused); } }"#,
    "if crate::hardware::broker::bootfb::is_boot_framebuffer(device_id) && surrendered() {",
    "return Err(ClaimError::Refused); }",
    "let epoch = next_epoch(); claims.push(Claim::making(pid, device_id, epoch)); Ok(epoch) }",
];

#[test]
fn the_kept_bound_is_asked_after_the_record_inside_both_holds() {
    let (a, e) = (flat(ASK), flat(ENTER));
    whole(&a, "admits", &ADMITS);
    whole(&e, "enter", &ENTERS);
    assert_eq!(num::<String>(&e, "KEEP_CAP", 2), format!("{} << 20", KEEP_CAP >> 20));
    assert!(KEEP_CAP.is_multiple_of(1 << 20), "the model's bound is another");
}
