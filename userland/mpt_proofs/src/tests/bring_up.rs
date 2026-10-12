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

//! The IOC brought to operational through the doorbell, every refusal, the
//! scan, and discovery.

use nonos_libc::{Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
use nonos_scsi::{Addr, Dir, Fault, Hba};

use crate::discover::found;
use crate::ioc;
use crate::model::sim::Unit;
use crate::model::{ioc as model, OPERATIONAL};

use super::util::{grant, up};

#[test]
fn facts_init_reply_frames_and_port_enable_in_order() {
    let (u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let s = m.0.borrow();
    assert_eq!(s.log[..2], [0x03, 0x02], "IOC FACTS then IOC INIT, through the doorbell");
    assert_eq!(s.state, OPERATIONAL);
    assert_eq!(s.mask, 0x9, "every interrupt masked");
    let init = s.init.as_ref().unwrap();
    assert_eq!(
        (init[0], init[3], init[5], init[6]),
        (0x04, 0x02, 16, 1),
        "the host driver, 16 devices, one bus"
    );
    assert_eq!(u16::from_le_bytes([init[12], init[13]]), 128, "128-byte reply frames");
    assert_eq!(&init[16..28], &[0; 12], "every high address half zero: the grant is below 4 GiB");
    assert_eq!(
        (u16::from_le_bytes([init[40], init[41]]), u16::from_le_bytes([init[42], init[43]])),
        (0x0105, 0x0E00)
    );
    assert_eq!(s.free.len(), 8, "every reply frame handed over and back");
    assert_eq!(u.hba.max_devices, 16);
}

#[test]
fn an_operational_ioc_is_reset_first() {
    let m = model(vec![]);
    m.0.borrow_mut().state = OPERATIONAL;
    let g = grant();
    ioc::start(&m, g.user_va, g.device_addr).expect("IOC up");
    assert_eq!(m.0.borrow().log[0], 0x40, "a message unit reset before the facts");
}

#[test]
fn refusals() {
    let g = grant();
    let m = model(vec![]);
    m.0.borrow_mut().state = OPERATIONAL;
    m.0.borrow_mut().stuck = true;
    assert_eq!(ioc::start(&m, g.user_va, g.device_addr), Err("IOC not ready after reset"));
    let m = model(vec![]);
    m.0.borrow_mut().refuse_init = true;
    assert_eq!(ioc::start(&m, g.user_va, g.device_addr), Err("IOC init refused"));
    let m = model(vec![]);
    m.0.borrow_mut().init_not_operational = true;
    assert_eq!(ioc::start(&m, g.user_va, g.device_addr), Err("IOC not operational after init"));
    let m = model(vec![]);
    m.0.borrow_mut().frame_dwords = 12;
    assert_eq!(ioc::start(&m, g.user_va, g.device_addr), Err("IOC request frames not served"));
}

#[test]
fn a_doorbell_interrupt_left_from_before_is_cleared_first() {
    let g = grant();
    let m = model(vec![]);
    m.0.borrow_mut().his |= 1;
    ioc::start(&m, g.user_va, g.device_addr).expect("IOC up past a stale interrupt");
}

#[test]
fn a_doorbell_in_use_is_left_alone() {
    let g = grant();
    let m = model(vec![]);
    m.0.borrow_mut().doorbell_busy = true;
    assert_eq!(ioc::start(&m, g.user_va, g.device_addr), Err("doorbell in use"));
}

#[test]
fn an_older_ioc_s_shorter_facts_are_enough() {
    let m = model(vec![]);
    m.0.borrow_mut().short_facts = true;
    let g = grant();
    assert_eq!(ioc::start(&m, g.user_va, g.device_addr).map(|f| f.max_devices), Ok(16));
}

#[test]
fn port_enable_answered_in_a_reply_frame() {
    let m = model(vec![]);
    m.0.borrow_mut().port_enable_address_reply = true;
    let g = grant();
    assert!(ioc::start(&m, g.user_va, g.device_addr).is_ok());
    assert_eq!(m.0.borrow().free.len(), 8, "the frame given back");
}

#[test]
fn the_scan_covers_the_iocs_devices_and_names_by_target_then_lun() {
    let mut t1 = Unit::disk(32, 512);
    t1.luns = vec![0, 2];
    let m = model(vec![((1, 2), Unit::cd(10)), ((1, 0), t1), ((7, 0), Unit::disk(16, 512))]);
    m.0.borrow_mut().max_devices = 8;
    let u = super::util::up_on(&m);
    let at: Vec<(u16, u16)> = u.disks.iter().flatten().map(|d| (d.at.target, d.at.lun)).collect();
    assert_eq!(at, vec![(1, 0), (1, 2), (7, 0)]);
    let s = m.0.borrow();
    let targets: std::collections::BTreeSet<u8> = s.ios.iter().map(|i| i.target).collect();
    assert_eq!(targets.len(), 8, "targets 0 to 7, as the facts say");
}

#[test]
fn a_target_past_the_iocs_devices_is_never_sent() {
    let m = model(vec![((0, 0), Unit::disk(16, 512))]);
    m.0.borrow_mut().max_devices = 8;
    let mut u = super::util::up_on(&m);
    let sent = m.0.borrow().ios.len();
    let at = Addr { target: 8, lun: 0 };
    assert_eq!(u.hba.exec(at, &[0; 6], Dir::None, 0).err(), Some(Fault::NoTarget));
    assert_eq!(m.0.borrow().ios.len(), sent, "refused before a frame went out");
}

fn rec(device: u16, bars: &[(u8, u64)]) -> DeviceRecord {
    let mut r = DeviceRecord {
        device_id: 5,
        bus_kind: BUS_KIND_PCI,
        vendor: 0x1000,
        device,
        bar_count: bars.len() as u8,
        ..DeviceRecord::default()
    };
    for (i, &(kind, size)) in bars.iter().enumerate() {
        r.bars[i] = Bar { kind, size };
    }
    r
}

#[test]
fn discovery_takes_both_parts_and_their_memory_bar() {
    let layout = [(BAR_KIND_PIO, 256), (BAR_KIND_MMIO, 0x4000), (BAR_KIND_MMIO, 0x10000)];
    assert_eq!(found(&rec(0x0030, &layout)).map(|f| f.bar), Some(1));
    assert_eq!(found(&rec(0x0054, &layout)).map(|f| f.bar), Some(1));
    assert!(found(&rec(0x0056, &layout)).is_none(), "an MPT2 part");
    assert!(found(&rec(0x0030, &[(BAR_KIND_PIO, 256)])).is_none());
}
