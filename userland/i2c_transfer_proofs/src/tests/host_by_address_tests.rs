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

//! The capsule on a kernel that still names a pad's host by address, as
//! every kernel before the host-by-id one does: the list shows every base to
//! every caller, and an I2C-HID record's bars[0].base is its platform host's
//! physical window, or 0 for a PCI host. Read by the shipping discovery,
//! such a base names no controller's id, and 0 names PCI id 0, the host
//! bridge, no I2C controller; it is not unresolved, so "I2Cn" does not
//! apply. Pairing fails closed: no controller is tried first, swept or
//! slowed for a pad, no pad but the bound one is in reach, and the remap
//! still comes from the controller's own record, which shows its window.
//! Neither half of the host-by-id change lands without the other.

use nonos_i2cmodel::{Bus, Config, Designware};
use nonos_libc::list_devices;

use super::declared::pad_at;
use super::listed::{discovered, order, table};
use super::multi_sku::named;
use super::records::{hid, lpss, platform, slow, I2C0, PLATFORM_WINDOW, WINDOW};
use super::remap_tests::{bring, remaps};
use crate::bench::{settled_on, GEMINI_LAKE_HZ as HZ};
use crate::regs::{self, Attached};
use crate::setup::order::standard_mode;
use crate::transaction::Presence;

fn core() -> Attached {
    regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad_at(0x15))))
}

#[test]
fn on_a_kernel_naming_hosts_by_address_no_pad_pairs_and_the_remap_is_right() {
    let pads = [hid(0, I2C0, 0x2A), slow(hid(PLATFORM_WINDOW, 0, 0x10)), hid(0, I2C0, 0)];
    list_devices(Ok(table(&[lpss(3), platform(4)], &pads)));
    let (cands, pads) = discovered();
    let _core = core();
    assert_eq!(order(&cands, &pads), [0, 1], "a controller was named");
    let speeds: Vec<_> = cands.iter().map(|c| standard_mode(c, &pads)).collect();
    assert_eq!(speeds, [false, false], "slowed for a pad named by address");
    for (dev, tg) in [(cands[0], pads[0]), (cands[1], pads[1])] {
        let d = settled_on(&dev, HZ, &tg, Presence::Absent, &pads).expect("bind");
        assert_eq!(named(&d, 2), [tg.addr; 2], "a pad paired by its host's address");
        assert_eq!(d.bound_addr, tg.addr);
    }
    assert!(regs::with(|c| c.bus().trace().is_empty()), "swept for a pad by address");
    let _core = core();
    assert_eq!(bring(&cands[0]), Ok(()));
    assert_eq!(remaps(), [WINDOW], "the window its own record shows");
}
