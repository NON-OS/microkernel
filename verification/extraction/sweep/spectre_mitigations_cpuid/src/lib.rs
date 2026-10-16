// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/hardening/spectre_mitigations/cpuid.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.
//!
//! `has_ibrs_ibpb`, `has_stibp` and `has_ssbd` lost their kernel functions in
//! 5f47bee28: the controls are read from both vendors' leaves by
//! `cpuid_spec::read` now. Those three wrappers take the field the kernel's
//! own callers take (`ibrs.rs`, `stibp.rs` and `ssbd.rs` test
//! `cpuid_spec::read().ibrs`, `.stibp` and `.ssbd`). IBPB is its own control
//! since that commit, `SpecControls::ibpb`, and `has_ibrs_ibpb` does not
//! answer for it.
//!
//! The modules are mounted at the crate root because `spectre_mitigations`'
//! files name one another as `super::<sibling>`, and the crate root is
//! `super` of each of them here.

// `has_enhanced_ibrs` reads IA32_ARCH_CAPABILITIES with `rdmsr`. That is
// hardware access and outside the extraction's scope, so no entry point
// reaches it; it is compiled only because it shares this file.
#[allow(dead_code)]
#[path = "../../../../../src/security/hardening/spectre_mitigations/cpuid.rs"]
pub mod cpuid;

#[path = "../../../../../src/security/hardening/spectre_mitigations/cpuid_spec.rs"]
pub mod cpuid_spec;

// Mounted so `cpuid::has_enhanced_ibrs` resolves `super::msr::rdmsr`. The MSR
// accessors are inline assembly and no entry point reaches them.
#[allow(dead_code)]
#[path = "../../../../../src/security/hardening/spectre_mitigations/msr.rs"]
mod msr;

// Mounted so `cpuid::has_enhanced_ibrs` resolves `super::constants::*`. Only
// `MSR_IA32_ARCH_CAPABILITIES` and `ARCH_CAP_IBRS_ALL` are named from here.
#[allow(dead_code)]
#[path = "../../../../../src/security/hardening/spectre_mitigations/constants.rs"]
mod constants;

pub fn has_ibrs_ibpb() -> bool {
    cpuid_spec::read().ibrs
}

pub fn has_stibp() -> bool {
    cpuid_spec::read().stibp
}

pub fn has_ssbd() -> bool {
    cpuid_spec::read().ssbd
}

pub fn has_l1d_flush() -> bool {
    cpuid::has_l1d_flush()
}

pub fn has_md_clear() -> bool {
    cpuid::has_md_clear()
}

pub fn has_arch_capabilities() -> bool {
    cpuid::has_arch_capabilities()
}

pub fn is_amd() -> bool {
    cpuid::is_amd()
}
