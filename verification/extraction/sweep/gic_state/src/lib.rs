// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/gic/state.rs` and
//! `src/arch/aarch64/gic/version.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/gic/state.rs"]
pub mod state;

// `state.rs` names `super::version::GicVersion`, so that path has to resolve at
// the crate root as it does in the kernel's `gic` module. The file is mounted as
// `gic_version` and aliased to `version` rather than mounted as `version`:
// Aeneas names the enum after its defining module, and `set_layout` has a
// parameter called `version`, which in Lean shadows a namespace `version` and
// turns the match arm `version.GicVersion.V2` into a field access on the
// parameter, an invalid pattern.
#[path = "../../../../../src/arch/aarch64/gic/version.rs"]
pub mod gic_version;
use gic_version as version;

// The kernel renamed `set_bases(dist, redist)` to
// `set_layout(version, dist, cpu, redist)` when it added the GICv2 backend.
// The wrapper keeps the name `crates.json` lists and forwards to the new
// function unchanged.
pub fn set_bases(version: version::GicVersion, dist: u64, cpu: u64, redist: u64) {
    state::set_layout(version, dist, cpu, redist)
}

pub fn dist_base() -> u64 {
    state::dist_base()
}

pub fn redist_base() -> u64 {
    state::redist_base()
}

pub fn cpu_base() -> u64 {
    state::cpu_base()
}

pub fn version() -> Option<version::GicVersion> {
    state::version()
}
