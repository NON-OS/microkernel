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

//! With no unit to confine a device, every verdict is asked with the
//! port-only rule (H-02b), and applied past the refusal of a device behind
//! an external-facing port (unconfined.rs).

const ATTACH: &str = include_str!("../../../../src/hardware/broker/confine/attach.rs");
const CAPSULE: &str = include_str!("../../../../src/hardware/broker/confine/capsule.rs");
const REFUSE: &str = include_str!("../../../../src/hardware/broker/confine/refuse.rs");
const UNCONFINED: &str = include_str!("../../../../src/hardware/broker/confine/unconfined.rs");

#[test]
fn every_claim_no_unit_confines_asks_the_verdict_with_port_only() {
    let asked = "Ok(()), port_only, (OPT_IN, PRODUCTION))";
    assert!(ATTACH.contains(&format!("verdict(false, false, {asked}")));
    assert!(ATTACH.contains(&format!("let v = verdict(true, false, {asked};")));
    assert!(ATTACH.contains("return Ok(unconfined(device_id, v)?);"));
    let held = "let v = verdict(true, true, Err(true), port_only, (OPT_IN, PRODUCTION));";
    assert!(CAPSULE.contains(held));
    assert!(CAPSULE.contains("unconfined(device_id, v)?;"));
    // The verdict applied past the external-port refusal (unconfined.rs).
    assert!(UNCONFINED.contains("    without_domain(v)\n}"));
    let no_dma = REFUSE.find("Verdict::NoDma => {").expect("NoDma refused");
    let ok = REFUSE[no_dma..].find("return Ok(());").expect("NoDma not admitted");
    assert!(ok < REFUSE[no_dma..].find("Verdict::Unconfined").unwrap());
}
