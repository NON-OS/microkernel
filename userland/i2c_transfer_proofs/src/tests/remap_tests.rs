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

//! An LPSS function's remap, as bring-up programs it from the device list.
//! Discovery lists every controller before any claim, when no window's base
//! shows, so bring-up reads the function's own record back once it holds
//! it, through the shipped claimed_base; a platform controller has no LPSS
//! block to remap, and is not read back.

use nonos_i2cmodel::{Bus, Config, Designware};
use nonos_libc::list_devices;

use super::listed::{discovered, list, table};
use super::records::{lpss, platform, WINDOW};
use crate::constants::{LPSS_PRIV_REMAP_HI, LPSS_PRIV_REMAP_LO, LPSS_PRIV_RESETS};
use crate::discover::{claimed_base, Found};
use crate::regs::{self, Attached, Regs};
use crate::setup::brought_up::brought_up;

pub fn core() -> Attached {
    regs::attach(Designware::new(Config::LPSS, Bus::default()))
}

/// `dev` brought up as bring_up_one.rs does once it is claimed and mapped.
pub fn bring(dev: &Found) -> Result<(), &'static str> {
    brought_up(dev, claimed_base, Regs::new(0, 0x1000), false).map(|_| ())
}

/// Each remap the attached core was programmed with, low half then high.
pub fn remaps() -> Vec<u64> {
    let (lo, hi) =
        regs::with(|c| (c.writes_to(LPSS_PRIV_REMAP_LO), c.writes_to(LPSS_PRIV_REMAP_HI)));
    lo.into_iter().zip(hi).map(|(lo, hi)| u64::from(hi) << 32 | u64::from(lo)).collect()
}

#[test]
fn an_lpss_function_is_remapped_to_its_window_as_its_own_record_reads_once_claimed() {
    let board = table(&[lpss(0), lpss(1), platform(2)], &[]);
    list(&board, None);
    let (found, _) = discovered();
    assert_eq!(found.iter().map(|f| f.device_id).collect::<Vec<_>>(), [0, 1, 2]);
    assert_eq!(claimed_base(1), Some(0), "before the claim the list shows no base");
    list(&board, Some(1));
    let core_1 = core();
    assert_eq!(bring(&found[1]), Ok(()));
    assert_eq!(remaps(), [WINDOW], "its window, both halves");
    drop(core_1);
    list_devices(Err(-14));
    let _core = core();
    regs::with(|c| c.write32(LPSS_PRIV_RESETS, 0x3));
    assert_eq!(bring(&found[2]), Ok(()), "a platform controller was read back");
    assert!(remaps().is_empty(), "a platform controller was remapped");
}
