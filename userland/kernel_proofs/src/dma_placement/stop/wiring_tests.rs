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

//! Where a release reaches the SD host's stop (F-128): every stop of a
//! claim asks it before any Bus Master write, a give-back and an exit stop
//! the device before its DMA grants go, an ACPI host is reset by the rule
//! acpi_stop names, and a released frame of a host still running is kept.

use super::wiring_text::{before, body};

const QUIESCE: &str = include_str!("../../../../../src/hardware/broker/claim/quiesce.rs");
const OTHER: &str = include_str!("../../../../../src/hardware/broker/claim/quiesce_other.rs");
const GIVE_BACK: &str = include_str!("../../../../../src/hardware/broker/give_back.rs");
const EXIT: &str = include_str!("../../../../../src/process/exit/devices.rs");
const HOSTS: &str = include_str!("../../../../../src/hardware/broker/dma/sd_hosts.rs");
const TEARDOWN: &str = include_str!("../../../../../src/hardware/broker/dma/teardown.rs");
const ADD: &str = include_str!("../../../../../src/hardware/broker/acpi/platform_add.rs");

#[test]
fn every_claim_stop_asks_the_sd_host_before_bus_master() {
    for name in ["stop_bus_master", "stop_bus_master_quietly"] {
        let stop = "ifsuper::quiesce_other::stopped(device_id){return;}";
        assert!(before(&body(QUIESCE, name), stop, "pci_index::lookup(device_id)"), "{name}");
    }
    let both = "described::stop(device_id)||crate::hardware::broker::dma::stop_sd_host(device_id)";
    assert!(body(OTHER, "stopped").contains(both), "claim/quiesce_other.rs");
}

#[test]
fn a_give_back_and_an_exit_stop_the_device_before_its_grants_go() {
    let g = body(GIVE_BACK, "give_back");
    assert!(before(&g, "quiesce_held_device(pid,device_id,epoch)", "dma_release_for_device("));
    let e = body(EXIT, "release_if_last");
    assert!(before(&e, "claims_exit_begin(holder)", "dma_release_all_for_pid(holder"));
}

#[test]
fn an_acpi_sd_host_is_reset_and_one_still_running_keeps_its_frames() {
    let s = body(HOSTS, "stop_sd_host");
    assert!(s.contains("ifbus!=BUS_ACPI||acpi_stop(class)!=Stop::SdhciResetAll{returnfalse;}"));
    assert!(s.contains("regs.map_or(Outcome::Unreachable,|mutr|reset_all(&mutr))"), "{s}");
    assert!(
        s.contains("running.retain(|&d|d!=device_id);if!frees(outcome){running.push(device_id);")
    );
    let t = body(TEARDOWN, "teardown");
    let keep = "||(released&&super::sd_hosts::running(g.device_id));";
    assert!(before(&t, keep, "ifsuper::kept_rule::keeps(pci,g.confined,released)||host{"));
    let a = body(ADD, "add");
    assert!(a.contains("letsd=r.class==ids::BLOCK;"), "{a}");
    let regs = "letregs=p.windows.first().copied().filter(|_|fenced.is_none());";
    assert!(a.contains(&format!("ifsd{{{regs}super::super::dma::note_sd_host(id,regs);}}")));
}
