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

//! Find the device and prove it delivers before anything counts on it.

use core::sync::atomic::Ordering;

use super::device::VirtioRngDevice;
use super::state::{VIRTIO_RNG, VIRTIO_RNG_AVAILABLE};
use crate::bus::pci;
use crate::drivers::pci::PciAddress;
use crate::sys::serial;

const VENDOR: u16 = 0x1AF4;
/// The transitional id. A modern-only device (0x1044) has no legacy
/// registers; its capsule drives it.
const TRANSITIONAL: u16 = 0x1005;

pub fn init() -> Result<(), &'static str> {
    if !pci::is_init() {
        return Err("virtio-rng: PCI not initialized");
    }
    for i in 0..pci::device_count() {
        let Some(dev) = pci::get_device(i) else { continue };
        if dev.vendor_id != VENDOR || dev.device_id != TRANSITIONAL {
            continue;
        }
        if dev.bar0 & 1 == 0 {
            serial::println(b"[VIRTIO-RNG] no legacy I/O BAR: left to its capsule");
            continue;
        }
        let address =
            PciAddress { segment: 0, bus: dev.bus, device: dev.device, function: dev.function };
        pci::enable_io_space(dev.bus, dev.device, dev.function);
        pci::enable_bus_master(dev.bus, dev.device, dev.function);
        let mut rng = match VirtioRngDevice::probe((dev.bar0 & 0xFFFC) as u16, address) {
            Ok(rng) => rng,
            Err(e) => {
                serial::print(b"[VIRTIO-RNG] init failed: ");
                serial::println(e.as_bytes());
                continue;
            }
        };
        // Counted as a source only once it has answered with bytes.
        let mut probe = [0u8; 16];
        if !matches!(rng.get_random_bytes(&mut probe), Ok(n) if n > 0) {
            serial::println(b"[VIRTIO-RNG] delivered nothing: not a source");
            continue;
        }
        probe.fill(0);
        *VIRTIO_RNG.lock() = Some(rng);
        VIRTIO_RNG_AVAILABLE.store(true, Ordering::SeqCst);
        serial::println(b"[VIRTIO-RNG] ready");
        return Ok(());
    }
    Err("virtio-rng: no device found")
}
