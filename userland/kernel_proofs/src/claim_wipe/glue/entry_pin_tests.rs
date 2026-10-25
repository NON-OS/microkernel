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


//! A claim asks the wipe record, and with it the kept bound, of every PCI
//! function (L78, L80): claim.rs `claim` enters with what the record is
//! told, which holder.rs `fresh` gives for any function the PCI index knows.
//! Both pinned (whole), so a claim that skips the question is seen.

use super::code::flat;
use super::whole::{fns, whole};

const CLAIM: &str = include_str!("../../../../../src/hardware/broker/claim/claim.rs");
const FRESH: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/holder.rs");

/// claim.rs `claim` up to its entry, the firmware's flash refused first.
const CLAIMS: [&str; 5] = [
    "pub fn claim(pid: u32, device_id: u64) -> Result<u64, ClaimError> {",
    "if super::firmware::is_firmware(device_id) { return Err(ClaimError::Firmware); }",
    "hold::admits(pid, device_id)?; let fresh = super::wipe::fresh(pid, device_id);",
    "let covers = crate::hardware::broker::bootfb::covers(device_id);",
    "let epoch = super::enter::enter(pid, device_id, fresh.as_ref())?;",
];

const FRESHS: [&str; 5] = [
    "pub(in crate::hardware::broker) fn fresh(pid: u32, device_id: u64) -> Option<Fresh> {",
    "let handle = crate::hardware::broker::pci_index::lookup(device_id)?;",
    "let served = crate::services::registry::endpoints_of(pid);",
    "let by = name_of(served.iter().map(|e| e.name.as_str()));",
    "Some(Fresh { by, caps: handle.reset, address: handle.address }) }",
];

#[test]
fn a_claim_asks_the_record_of_every_pci_function() {
    let c = flat(CLAIM);
    assert_eq!(fns(&c, "claim"), 1, "a twin claim");
    let head = &c[c.find(CLAIMS[0]).expect("claim")..];
    assert!(head.starts_with(&CLAIMS.join(" ")), "a claim that never asks the record");
    whole(&flat(FRESH), "fresh", &FRESHS);
}
