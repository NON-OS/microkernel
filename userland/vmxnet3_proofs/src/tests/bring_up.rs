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

//! A good device, brought up: revision 1 selected, the device reset first,
//! the drawn address set, the layout accepted, every ring 0 slot but one
//! handed over, and the filter applied after activation.

use super::util::{commands, drawn, up};
use crate::bench::{Bench, CONTROL_DEV, RX_DEV};
use crate::model::{take, Event};

#[test]
fn the_device_accepts_the_layout_and_gets_the_drawn_address() {
    let mut bench = Bench::new();
    let driver = up(&mut bench);
    assert_eq!(driver.mac, drawn());
    let d = take();
    assert_eq!(d.refusal, None);
    assert!(d.active && d.masked);
    assert_eq!((d.vrrs, d.uvrs, d.mac, d.dsa, d.mtu), (1, 1, drawn(), CONTROL_DEV, 1500));
    assert_eq!((d.tx.base, d.tx_comp.base), (CONTROL_DEV + 0xA00, CONTROL_DEV + 0xC00));
    assert_eq!(
        (d.rx.base, d.rx1.base, d.rx_comp.base),
        (CONTROL_DEV + 0xE00, CONTROL_DEV + 0x1000, CONTROL_DEV + 0x1200)
    );
    assert_eq!(
        (d.tx.size, d.tx_comp.size, d.rx.size, d.rx1.size, d.rx_comp.size),
        (32, 32, 32, 32, 64)
    );
    assert_eq!((d.rx_mode, d.vf0), (0x05, 1), "unicast and broadcast, VLAN 0");
    for i in 0..32u32 {
        let desc = d.rx.base + i as u64 * 16;
        let owned = d.mem.u32(desc + 8) >> 31 == 1;
        assert_eq!(owned, i < 31, "slot {i}: all but the last handed over");
        if owned {
            assert_eq!(d.mem.u64(desc), RX_DEV + i as u64 * 2048);
            assert_eq!(d.mem.u32(desc + 8) & 0x3FFF, 2048);
        }
        assert_eq!(d.mem.u32(d.rx1.base + i as u64 * 16 + 8), 0, "ring 1 stays empty");
    }
    assert!(d.log.contains(&Event::RxProd(31)) && d.log.contains(&Event::RxProd2(0)));
    assert!(nonos_libc::logged()[0].starts_with("driver.vmxnet3: up, link up 10000 Mb/s"));
    assert!(nonos_libc::given_back().is_empty());
}

#[test]
fn reset_then_activate_then_the_filter() {
    let mut bench = Bench::new();
    up(&mut bench);
    let cmds = commands(&take().log);
    let want = [0xCAFE_0002, 0xCAFE_0000, 0xCAFE_0003, 0xCAFE_0005, 0xCAFE_0004, 0xF00D_0002];
    assert_eq!(cmds, want);
}
