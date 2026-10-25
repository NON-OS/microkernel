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

//! bootfb's hold read whole but for its three lines of text: HELD set for
//! the boot framebuffer only, in the claims' lock, and waited out after it;
//! cleared only as that claim's entry goes, in the same lock, by the one
//! caller; and the line said on the way out picks the takeover's words
//! only while `taken()`.

use super::text::code;
use super::text_items::{flat, flat_body};
use super::text_uses::{calls, every_file};

const HOLD: &str = "hardware/broker/bootfb/hold.rs";
const GONE: &str = "hardware/broker/claim/gone.rs";

#[test]
fn the_hold_is_set_and_cleared_only_for_the_boot_framebuffer() {
    let h = flat(&code(HOLD));
    let head = [
        "externcratealloc;usealloc::vec::Vec;usespin::MutexGuard;",
        "usesuper::register::is_boot_framebuffer;usecrate::hardware::broker::claim::Claim;",
        "usecrate::kernel_core::init::framebuffer::{set_held,taken,wait_quiet};",
        "usecrate::sys::serial;",
        "pub(incrate::hardware::broker)fnadmitted(claims:MutexGuard<'static,Vec<Claim>>,",
        "id:u64){ifis_boot_framebuffer(id){set_held(true);drop(claims);wait_quiet();",
        "say(true);}}pub(incrate::hardware::broker)fnreleased(device_id:u64)->bool{",
        "letfb=is_boot_framebuffer(device_id);iffb{set_held(false);}fb}",
        "pub(incrate::hardware::broker)fnsay(held:bool){letline:&[u8]=ifheld{b\"",
    ]
    .concat();
    assert!(h.starts_with(&head), "{h}");
    let say = &h[head.len()..];
    assert_eq!(say.matches("\"}elseiftaken(){b\"").count(), 1, "{say}");
    assert_eq!(say.matches("else").count(), 2, "{say}");
    assert!(say.ends_with("\"};serial::println(line);}"), "{say}");
}

#[test]
fn held_is_cleared_in_the_entrys_own_hold_by_its_one_caller() {
    let remove = [
        "letgone=claims.remove(at);super::wipe::owe(gone.device_id,gone.epoch);",
        "letfb=crate::hardware::broker::bootfb::released(gone.device_id);drop(claims);",
        "iffb{crate::hardware::broker::bootfb::say(false);}",
    ];
    assert_eq!(flat_body(GONE, "fn remove("), remove.concat());
    let callers: Vec<String> = every_file()
        .into_iter()
        .filter(|(rel, code)| rel.starts_with("hardware/") && calls(code, "released") > 0)
        .map(|(rel, _)| rel)
        .collect();
    assert_eq!(callers, [HOLD, GONE]);
}
