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

//! claim.rs read for what the takeover's order leaves out: whether a claim
//! covers is decided once, by the cover rule, before its entry is made;
//! the stop comes after the last refusal, nothing returns between it and
//! the admit, and the takeover is begun and ended once each.

use super::text::code;
use super::text_items::{at, flat, flat_body};

const CLAIM: &str = "hardware/broker/claim/claim.rs";

#[test]
fn covers_is_the_cover_rules_answer_read_once_before_the_entry() {
    let c = flat_body(CLAIM, "pub fn claim(");
    let covers = at(&c, "letcovers=crate::hardware::broker::bootfb::covers(device_id);");
    assert!(at(&c, "letfresh=super::wipe::fresh(pid,device_id);") < covers);
    assert!(covers < at(&c, "letepoch=super::enter::enter(pid,device_id,fresh.as_ref())?;"));
    assert_eq!(c.matches("covers=").count(), 1, "covers decided twice");
}

#[test]
fn the_stop_follows_every_refusal_and_the_admit_follows_the_stop() {
    let c = flat_body(CLAIM, "pub fn claim(");
    let take = at(&c, "ifcovers{super::scanout::take(device_id);}");
    assert!(at(&c, "super::master_off::bus_master_off(device_id);") < take);
    let after = &c[take..];
    assert!(!after.contains("return") && !after.contains('?'), "a refusal past the stop");
    assert!(!after.contains("withdraw(pid,device_id,epoch);"), "withdrawn before its end");
    assert_eq!(c.matches("scanout::take(").count(), 1, "the console stopped twice");
    assert_eq!(c.matches("end_takeover()").count(), 1, "a takeover ended twice");
    let scanout = flat(&code("hardware/broker/claim/scanout.rs"));
    assert_eq!(scanout.matches("stop_drawing()").count(), 1, "the console stopped twice");
}
