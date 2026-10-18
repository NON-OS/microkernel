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

//! Who may draw: the first to take the primary surface, alone; anyone may
//! ask after health. And discovery finds only a whole SVGA II function.

use super::util::up;
use crate::bench::Bench;
use crate::discover::find_svga;
use crate::protocol::Request;
use crate::server::dispatch::dispatch;
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
use nonos_libc::{replies, set_devices, shares, Bar, DeviceRecord};

fn req(op: u16) -> Request {
    Request { op, flags: 0, request_id: 9 }
}

#[test]
fn only_the_first_to_take_the_surface_may_draw() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let mut tx = [0u8; 64];
    assert_eq!(dispatch(&mut d, 40, &req(0x0A), &[], &mut tx), Some(-16), "no owner yet");
    assert_eq!(dispatch(&mut d, 40, &req(0x0C), &[], &mut tx), None);
    let (pid, reply) = replies().pop().unwrap();
    assert_eq!((pid, reply.len()), (40, 20 + 4 + 32));
    assert_eq!(&reply[24..32], &0x5003u64.to_le_bytes());
    assert_eq!(&reply[36..40], &1024u32.to_le_bytes());
    assert_eq!(dispatch(&mut d, 41, &req(0x0C), &[], &mut tx), Some(-16), "taken");
    assert_eq!(dispatch(&mut d, 40, &req(0x0C), &[], &mut tx), None, "asked again");
    assert_eq!(shares(), [(3, 40, SURFACE_SHARE_WRITE)], "the owner alone, writable, once");
    assert_eq!(dispatch(&mut d, 41, &req(0x01), &[], &mut tx), Some(0));
    assert_eq!(dispatch(&mut d, 40, &req(0x7F), &[], &mut tx), Some(-38));
}

#[test]
fn discovery_wants_ports_then_two_memory_windows() {
    let bar = |kind, size| Bar { kind, size };
    let mut rec = DeviceRecord {
        device_id: 5,
        bus_kind: 1,
        vendor: 0x15AD,
        device: 0x0405,
        bar_count: 3,
        ..Default::default()
    };
    rec.bars[..3].copy_from_slice(&[bar(2, 16), bar(1, 16 << 20), bar(1, 2 << 20)]);
    let mut wrong = rec;
    wrong.bars[2] = bar(2, 16);
    set_devices(vec![wrong, rec]);
    let f = find_svga().expect("found");
    assert_eq!((f.device_id, f.fb_bytes, f.fifo_bytes), (5, 16 << 20, 2 << 20));
    assert_eq!(f.layout, crate::constants::pci::VMWARE_LAYOUT);
}
