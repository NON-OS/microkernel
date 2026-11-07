// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/smp/boot_claim.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/smp/boot_claim.rs"]
pub mod boot_claim;

pub fn bootclaim_new() -> boot_claim::BootClaim {
    boot_claim::BootClaim::new()
}

pub fn bootclaim_arm(this: boot_claim::BootClaim) {
    this.arm()
}

pub fn bootclaim_entered(this: boot_claim::BootClaim) -> bool {
    this.entered()
}

