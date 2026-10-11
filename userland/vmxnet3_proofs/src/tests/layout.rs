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

//! The shared area against vmxnet3_defs.h, byte by byte, independent of
//! the device model's own reading of it.

use crate::shared::{write_rx_queue, write_shared, write_tx_queue, Area, Rings, SharedConfig};

fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn quad(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

#[test]
fn driver_shared_fields_sit_where_the_header_puts_them() {
    let mut b = vec![0u8; 800];
    let cfg = SharedConfig { queue_desc_pa: 0x1234_5000, dd_pa: 0x9000, dd_len: 0x3000 };
    write_shared(&mut Area::new(&mut b), &cfg);
    assert_eq!(word(&b, 0), 3133079265, "VMXNET3_REV1_MAGIC");
    assert_eq!((word(&b, 16), word(&b, 20)), (1, 1), "vmxnet3RevSpt, uptVerSpt");
    assert_eq!(quad(&b, 24), 0, "no UPT features");
    assert_eq!((quad(&b, 32), quad(&b, 40)), (0x9000, 0x1234_5000), "ddPA, queueDescPA");
    assert_eq!((word(&b, 48), word(&b, 52), word(&b, 56)), (0x3000, 512, 1500));
    assert_eq!((b[62], b[63]), (1, 1), "one queue each way");
    assert_eq!((b[80], b[81], b[82]), (1, 1, 0), "autoMask, numIntrs, eventIntrIdx");
    assert_eq!(word(&b, 108), 1, "VMXNET3_IC_DISABLE_ALL");
    assert_eq!((word(&b, 120), word(&b, 136)), (0x05, 1), "rxMode, vfTable[0]");
    assert_eq!(word(&b, 696), 0, "ecr");
}

#[test]
fn queue_descriptors_put_each_ring_in_its_conf_slot() {
    let mut b = vec![0u8; 512];
    let r = Rings {
        tx: 0xA00,
        tx_data: 0x2000,
        tx_comp: 0xC00,
        rx: 0xE00,
        rx1: 0x1000,
        rx_comp: 0x1200,
    };
    let mut a = Area::new(&mut b);
    write_tx_queue(&mut a, 0, &r);
    write_rx_queue(&mut a, 256, &r);
    assert_eq!(
        (quad(&b, 16), quad(&b, 24), quad(&b, 32), quad(&b, 40)),
        (0xA00, 0x2000, 0xC00, u64::MAX)
    );
    assert_eq!((word(&b, 56), word(&b, 60), word(&b, 64), word(&b, 68)), (32, 32, 32, 0));
    assert_eq!((b[72], u16::from_le_bytes([b[74], b[75]])), (0, 128));
    let q = 256;
    assert_eq!((quad(&b, q + 16), quad(&b, q + 24), quad(&b, q + 32)), (0xE00, 0x1000, 0x1200));
    assert_eq!((word(&b, q + 56), word(&b, q + 60), word(&b, q + 64)), (32, 32, 64));
}
