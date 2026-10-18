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

//! Modern whenever the function presents it. A transitional
//! function, legacy I/O window and all, has its capabilities read and is
//! driven modern; only the modern transport can take ACCESS_PLATFORM. The
//! legacy registers are left for one that does not present every structure
//! its driver maps.

use super::broker::FakeBroker;
use super::space::{qemu_modern, qemu_transitional_bars, Space, QEMU_MODERN_BAR, TRANSITIONAL_NET};
use crate::error::VirtioError;
use crate::map::Need;
use crate::select::modern_caps;

/// QEMU's default virtio-rng-pci on q35: 1af4:1005, transitional.
const TRANSITIONAL_RNG: u16 = 0x1005;
const RNG: Need = Need { isr: false, device: false };
const NET: Need = Need { isr: false, device: true };

#[test]
fn a_transitional_rng_with_its_io_bar_is_read_and_driven_modern() {
    let bars = qemu_transitional_bars();
    let space = qemu_modern(TRANSITIONAL_RNG);
    let mut broker = FakeBroker::new(&space, &bars);
    let caps = modern_caps(&bars, &mut broker, RNG).expect("read").expect("modern");
    assert_eq!(broker.reads, 64, "the whole snapshot was read");
    assert_eq!(caps.common.map(|r| (r.bar, r.offset)), Some((QEMU_MODERN_BAR, 0)));
    assert_eq!(caps.notify_multiplier, 4);
}

#[test]
fn a_transitional_function_missing_a_structure_keeps_the_legacy_registers() {
    let bars = qemu_transitional_bars();
    let mut broker = FakeBroker::new(&Space::new(TRANSITIONAL_NET, 0), &bars);
    assert_eq!(modern_caps(&bars, &mut broker, RNG), Ok(None));
    let mut broker = FakeBroker::new(&Space::new(TRANSITIONAL_RNG, 0).without_cap_list(), &bars);
    assert_eq!(modern_caps(&bars, &mut broker, RNG), Ok(None));
    // Notify, then device configuration, placed in the legacy I/O BAR0.
    for (device, need, bar_at) in [(TRANSITIONAL_RNG, RNG, 0x74), (TRANSITIONAL_NET, NET, 0x64)] {
        let mut space = qemu_modern(device);
        space.put8(bar_at, 0);
        let mut broker = FakeBroker::new(&space, &bars);
        assert_eq!(modern_caps(&bars, &mut broker, need), Ok(None), "{device:#x}");
    }
}

#[test]
fn a_config_space_that_cannot_be_read_stops_the_attempt() {
    let bars = qemu_transitional_bars();
    let space = qemu_modern(TRANSITIONAL_RNG);
    for offset in [0x00, 0x34, 0x40, 0xFC] {
        let mut broker = FakeBroker::new(&space, &bars);
        broker.refuse_read = Some(offset);
        let refused = Err(VirtioError::ConfigRead);
        assert_eq!(modern_caps(&bars, &mut broker, RNG), refused, "{offset:#x}");
        assert!(broker.requests.is_empty(), "nothing is mapped");
    }
}
