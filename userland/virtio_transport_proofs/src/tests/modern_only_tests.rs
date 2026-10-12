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

//! A modern-only function (0x1040 and up) has no legacy registers: missing a
//! structure its driver maps, it is refused by name, never driven legacy.
//! virtio-rng has no device configuration (virtio 1.2 5.4.4), so 4.1.4.6
//! asks no device capability of it.

use super::broker::FakeBroker;
use super::space::{qemu_modern, qemu_modern_bars, Space};
use crate::error::VirtioError::{self, NoCommonCfg, NoDeviceCfg, NoIsrCfg, NoNotifyCfg};
use crate::map::Need;
use crate::select::modern_caps;

const MODERN_BLK: u16 = 0x1042;
const MODERN_RNG: u16 = 0x1044;
const MODERN_GPU: u16 = 0x1050;
const GPU: Need = Need { isr: false, device: true };
const BLK: Need = Need { isr: true, device: true };
const RNG: Need = Need { isr: false, device: false };
// Where `qemu_modern` keeps the BAR of common, ISR, device and notify.
const COMMON: usize = 0x44;
const ISR: usize = 0x54;
const DEVICE: usize = 0x64;
const NOTIFY: usize = 0x74;

/// `device` with one structure moved to BAR0, which it does not have.
fn lacking(device: u16, bar_at: usize) -> Space {
    let mut s = qemu_modern(device);
    s.put8(bar_at, 0);
    s
}

fn modern(space: &Space, need: Need) -> Result<bool, VirtioError> {
    let bars = qemu_modern_bars();
    let mut broker = FakeBroker::new(space, &bars);
    let got = modern_caps(&bars, &mut broker, need).map(|caps| caps.is_some());
    assert!(broker.requests.is_empty(), "the choice mapped a region");
    got
}

#[test]
fn a_modern_only_function_is_refused_by_the_structure_it_lacks() {
    for need in [GPU, BLK, RNG] {
        assert_eq!(modern(&lacking(MODERN_GPU, COMMON), need), Err(NoCommonCfg));
        assert_eq!(modern(&lacking(MODERN_GPU, NOTIFY), need), Err(NoNotifyCfg));
        assert_eq!(modern(&Space::new(MODERN_RNG, 0), need), Err(NoCommonCfg));
    }
    assert_eq!(modern(&lacking(MODERN_GPU, DEVICE), GPU), Err(NoDeviceCfg));
    assert_eq!(modern(&lacking(MODERN_BLK, ISR), BLK), Err(NoIsrCfg));
}

#[test]
fn a_modern_only_function_with_what_its_driver_maps_is_driven_modern() {
    // virtio-rng maps neither device configuration nor ISR.
    assert_eq!(modern(&lacking(MODERN_RNG, DEVICE), RNG), Ok(true));
    assert_eq!(modern(&lacking(MODERN_RNG, ISR), RNG), Ok(true));
    assert_eq!(modern(&lacking(MODERN_RNG, DEVICE), GPU), Err(NoDeviceCfg));
    for (device, need) in [(MODERN_GPU, GPU), (MODERN_BLK, BLK), (MODERN_RNG, RNG)] {
        assert_eq!(modern(&qemu_modern(device), need), Ok(true), "{device:#x}");
    }
}
