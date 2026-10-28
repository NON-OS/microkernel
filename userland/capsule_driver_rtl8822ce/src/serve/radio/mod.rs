// NONOS Operating System (AGPL-3.0-or-later)

//! The radio the serve loop holds. Building it from the mapped chip (`up`)
//! is the RTL8821CE's; the PHY bring-up it calls (`phy`) is the 8822C's.

mod phy;
#[path = "../../../../capsule_driver_rtl8821ce/src/serve/radio/up.rs"]
mod up;

pub(super) use up::{build_radio, Radio, RadioUp};
