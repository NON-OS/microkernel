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

// A modern virtio-net's MAC comes out of its device configuration region,
// which the device sizes. These run the shipping read against regions in
// host memory: the common configuration (for the config generation) and a
// device region of a chosen length.

use nonos_virtio::{Mmio, VirtioError};

use crate::constants::{VIRTIO_NET_F_MAC, VIRTIO_NET_F_STATUS};
use crate::modern::config::read_mac;

const MAC: u64 = 1 << VIRTIO_NET_F_MAC;
const STATUS: u64 = 1 << VIRTIO_NET_F_STATUS;
const DEVICE_AT: usize = 0x800;

#[repr(align(4096))]
struct Page([u8; 4096]);

fn page() -> Box<Page> {
    let mut p = Box::new(Page([0; 4096]));
    p.0[DEVICE_AT..DEVICE_AT + 8].copy_from_slice(&[0x52, 0x54, 0x00, 0x12, 0x34, 0x56, 1, 0]);
    p
}

fn regions(page: &mut Page, device_len: usize) -> (Mmio, Mmio) {
    let base = page.0.as_mut_ptr();
    // SAFETY: both ranges lie inside the page, which outlives the test.
    unsafe { (Mmio::new(base, 0x38), Mmio::new(base.add(DEVICE_AT), device_len)) }
}

#[test]
fn the_mac_is_the_first_six_bytes_of_the_device_region() {
    let mut p = page();
    let (common, device) = regions(&mut p, 0x100);
    assert_eq!(read_mac(&common, device, MAC | STATUS), Ok([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]));
}

#[test]
fn without_the_mac_feature_the_address_is_zero_as_on_the_legacy_path() {
    let mut p = page();
    let (common, device) = regions(&mut p, 0x100);
    assert_eq!(read_mac(&common, device, STATUS), Ok([0; 6]));
}

#[test]
fn a_device_region_shorter_than_the_promised_fields_is_refused() {
    let mut p = page();
    let short = Err(VirtioError::DeviceCfgShort.message());
    for len in 0..8 {
        let (common, device) = regions(&mut p, len);
        // STATUS promises the status word after the MAC: eight bytes.
        assert_eq!(read_mac(&common, device, MAC | STATUS), short, "len {len}");
        // The MAC alone needs six.
        let mac_only = read_mac(&common, device, MAC);
        assert_eq!(mac_only.is_ok(), len >= 6, "len {len}");
    }
}
