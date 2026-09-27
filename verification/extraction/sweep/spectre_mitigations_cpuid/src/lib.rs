// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/hardening/spectre_mitigations/cpuid.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/hardening/spectre_mitigations/cpuid.rs"]
pub mod cpuid;

pub fn has_ibrs_ibpb() -> bool {
    cpuid::has_ibrs_ibpb()
}

pub fn has_stibp() -> bool {
    cpuid::has_stibp()
}

pub fn has_ssbd() -> bool {
    cpuid::has_ssbd()
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

