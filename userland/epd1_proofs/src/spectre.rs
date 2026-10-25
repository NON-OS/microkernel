// NONOS Operating System (AGPL-3.0-or-later)
//! The speculation decision, mounted whole: which CPUs it calls exposed is
//! which CPUs turn the trampoline on.
#[path = "../../../src/arch/aarch64/security/spectre/answer.rs"]
pub mod answer;
#[path = "../../../src/arch/aarch64/security/spectre/branch.rs"]
pub mod branch;
#[path = "../../../src/arch/aarch64/security/spectre/cores.rs"]
pub mod cores;
#[path = "../../../src/arch/aarch64/security/spectre/decide.rs"]
pub mod decide;
#[path = "../../../src/arch/aarch64/security/spectre/history.rs"]
pub mod history;
#[path = "../../../src/arch/aarch64/security/spectre/ids.rs"]
pub mod ids;
#[path = "../../../src/arch/aarch64/security/spectre/plan.rs"]
pub mod plan;
