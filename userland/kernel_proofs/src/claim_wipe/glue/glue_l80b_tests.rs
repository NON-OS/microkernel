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

//! The L80 glue the bit and count rules stand behind. Device Control 2's
//! firmware value is read once at enumeration, before any claim, from a
//! version 2 PCIe capability, and the put back writes it through the rule;
//! a kept run the registry's let-go frees is reported to its caller as one.

use super::code::flat;

const CAPS: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/caps.rs");
const CHECK: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/check.rs");
const INIT: &str = include_str!("../../../../../src/hardware/broker/table/init.rs");
const KEPT: &str = include_str!("../../../../../src/hardware/broker/dma/kept.rs");

#[test]
fn device_control_2_is_read_at_enumeration_and_put_back_through_the_rule() {
    let c = flat(CAPS);
    assert!(c.contains("let v2 = pcie.filter(|p| p.1 >= 2);"), "read from any version");
    let read = "let devctl2_boot = v2.and_then(|(o, _)| cfg.read16(u16::from(o) + 0x28).ok());";
    assert!(c.contains(read), "Device Control 2 not read where it sits");
    assert!(c.contains("devctl2: v2.is_some(), devctl2_boot: devctl2_boot.unwrap_or(0),"));
    assert!(flat(INIT).contains("reset: reset_caps(dev),"), "not read at enumeration");
    let put = "cfg.write16(devctl + 0x20, devctl2_after(d, w.caps.devctl2_boot));";
    assert!(flat(CHECK).contains(put), "Device Control 2 put back as the holder left it");
}

#[test]
fn a_let_go_that_frees_a_kept_run_says_so() {
    let k = flat(KEPT);
    let defer = &k[k.find("pub(super) fn defer(").expect("defer")..];
    let defer = &defer[..defer.find("pub(in crate::hardware::broker) fn kept_bytes").expect("end")];
    assert!(defer.contains("-> Option<bool> {"), "defer cannot tell a kept run from another");
    assert!(defer.contains(".position(|k| (k.start, k.len) == (start, len))?;"), "not kept");
    let free = "if act == Act::Free { all.remove(at); return Some(false); }";
    assert!(defer.contains(free), "a kept run freed by the let-go not reported as one");
    assert!(defer.contains("all[at].run = run; Some(true) }"), "a waiting run reported freed");
}
