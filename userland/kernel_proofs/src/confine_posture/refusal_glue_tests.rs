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

//! The kernel acts on the refusal: the posture decision stops the boot with
//! its line, the AgentProof build is the gauntlet's, and the gauntlet runs
//! where an IOMMU is in service, so the refusal never stops its own runs.

const DECIDE: &str = include_str!("../../../../src/memory/iommu/no_iommu.rs");
const INIT: &str = include_str!("../../../../src/kernel_core/init/entry/microkernel_init.rs");
const CARGO: &str = include_str!("../../../../Cargo.toml");
const CONFIG: &str = include_str!("../../../../tools/nix/config.nix");
const GAUNTLET: &str = include_str!("../../../../tools/agentproof_gauntlet/run.py");

fn decide() -> &'static str {
    let at = DECIDE.find("pub fn decide(").expect("decide");
    &DECIDE[at..at + DECIDE[at..].find("\n}\n").expect("end")]
}

#[test]
fn the_posture_decision_stops_the_boot_with_the_line() {
    let d = decide();
    assert!(d.contains("stops(false, AGENTPROOF, hardened, air_gapped)"));
    assert!(d.contains("crate::boot::stop(\"IOMMU\", needs.line());"));
    assert!(d.find("[IOMMU] posture=").unwrap() < d.find("boot::stop(").unwrap());
    assert!(d.contains("refusing_boot(hardened, air_gapped) || AGENTPROOF"), "never taken first");
    assert!(DECIDE.contains("const AGENTPROOF: bool = cfg!(feature = \"nonos-agentproof\");"));
}

#[test]
fn it_runs_before_any_driver_can_start() {
    let dma = INIT.find("init_dma_protection();").expect("dma");
    let runtime = INIT.find("init_process_runtime();").expect("runtime");
    assert!(dma < runtime, "the boot stops before a process, a driver among them, exists");
}

#[test]
fn the_gauntlet_is_the_agentproof_build_and_runs_with_an_iommu() {
    assert!(CARGO.contains("\nnonos-agentproof = []\n"));
    let g = &CONFIG[CONFIG.find("    gauntlet = {").expect("gauntlet")..];
    let kernel = &g[g.find("kernel = [").unwrap()..g.find("];").unwrap()];
    assert!(kernel.contains("\"nonos-agentproof\""));
    assert!(GAUNTLET.contains("\"--lane\", \"iommu=intel\""));
}
