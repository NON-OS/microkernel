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

//! The kernel acts on the refusal: the posture decision holds it, the boot
//! stops with its line once the panel can show it and before any process
//! exists, the AgentProof build is the gauntlet's, and the gauntlet runs where
//! an IOMMU is in service, so the refusal never stops its own runs.

const DECIDE: &str = include_str!("../../../../src/memory/iommu/no_iommu.rs");
const INIT: &str = include_str!("../../../../src/kernel_core/init/entry/microkernel_init.rs");
const FB: &str = include_str!("../../../../src/kernel_core/init/entry/init_arch_framebuffer.rs");
const STOP: &str = include_str!("../../../../src/memory/iommu/no_iommu_stop.rs");
const NOTICE: &str = include_str!("../../../../src/sys/boot_log/notice_screen.rs");
const CARGO: &str = include_str!("../../../../Cargo.toml");
const CONFIG: &str = include_str!("../../../../tools/nix/config.nix");
const GAUNTLET: &str = include_str!("../../../../tools/agentproof_gauntlet/run.py");

fn decide() -> &'static str {
    let at = DECIDE.find("pub fn decide(").expect("decide");
    &DECIDE[at..at + DECIDE[at..].find("\n}\n").expect("end")]
}

#[test]
fn the_posture_decision_holds_the_refusal() {
    let d = decide();
    assert!(d.contains("stops(false, AGENTPROOF, hardened, air_gapped)"));
    assert!(d.contains("super::no_iommu_stop::hold(needs);"));
    assert!(d.find("[IOMMU] posture=").unwrap() < d.find("hold(needs)").unwrap());
    assert!(d.contains("refusing_boot(hardened, air_gapped) || AGENTPROOF"), "never taken first");
    assert!(DECIDE.contains("const AGENTPROOF: bool = cfg!(feature = \"nonos-agentproof\");"));
}

#[test]
fn the_held_refusal_stops_the_boot_once_the_panel_is_mapped() {
    assert!(STOP.contains("crate::boot::stop(\"IOMMU\", needs.line());"));
    let mapped = FB.rfind("ArchSpecificHandoff::Riscv64").expect("the last arm");
    let stop = FB.find("crate::memory::iommu::stop_if_no_iommu_refused();").expect("stop");
    assert!(mapped < stop, "after the mapping, so the band is drawn");
    let at = |s: &str| INIT.find(s).unwrap_or_else(|| panic!("{s}"));
    assert!(at("init_dma_protection();") < at("init_arch_framebuffer(handoff);"));
    assert!(at("init_arch_framebuffer(handoff);") < at("init_device_routing();"));
    assert!(at("init_device_routing();") < at("init_process_runtime();"), "before any process");
}

#[test]
fn the_gauntlet_is_the_agentproof_build_and_runs_with_an_iommu() {
    assert!(CARGO.contains("\nnonos-agentproof = []\n"));
    let g = &CONFIG[CONFIG.find("    gauntlet = {").expect("gauntlet")..];
    let kernel = &g[g.find("kernel = [").unwrap()..g.find("];").unwrap()];
    assert!(kernel.contains("\"nonos-agentproof\""));
    assert!(GAUNTLET.contains("\"--lane\", \"iommu=intel\""));
}

#[test]
fn the_stop_is_the_only_thing_on_the_panel() {
    let band = NOTICE.find("fill_rect(fb, 0, 0, fb.width, band, BAND_COLOR);").expect("band");
    let blank = NOTICE.find("fill_rect(fb, 0, band, fb.width, fb.height - band, BLANK);");
    assert!(blank.is_some_and(|b| b > band), "what the loader drew is cleared under the band");
    assert!(NOTICE.contains(".min(fb.height);"), "the band never runs past the panel");
}
