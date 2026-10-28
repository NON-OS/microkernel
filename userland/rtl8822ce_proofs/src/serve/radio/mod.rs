// NONOS Operating System (AGPL-3.0-or-later)
//! The radio, mounted as the capsule mounts it, with the PHY entry's proofs.

#[path = "../../../../capsule_driver_rtl8822ce/src/serve/radio/phy.rs"]
mod phy;
#[cfg(test)]
mod phy_tests;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/radio/up.rs"]
mod up;

pub(super) use up::{build_radio, Radio, RadioUp};
