// NONOS Operating System (AGPL-3.0-or-later)
//! Devices as the tests want them, and what resets one.

use super::device::Device;
use super::ep0::Ep0;

impl Device {
    /// A full-speed device with EP0 of `mps0` and 64-byte bulk packets.
    pub fn full(mps0: u8) -> Self {
        let desc = [18, 1, 0x10, 1, 0, 0, 0, mps0, 0x34, 0x12, 0x78, 0x56, 0, 1, 1, 2, 0, 1];
        let config = vec![9, 2, 32, 0, 1, 1, 0, 0x80, 50, 9, 4, 0, 0, 2, 8, 6, 80, 0];
        let config = [config, vec![7, 5, 0x82, 2, 64, 0, 0, 7, 5, 3, 2, 64, 0, 0]].concat();
        Device { mps0, desc, config, bulk_mps: 64, ..Default::default() }
    }

    pub fn low() -> Self {
        Device { low_speed: true, ..Device::full(8) }
    }

    /// A bus reset: address 0, every endpoint back to DATA0.
    pub fn reset(&mut self) {
        (self.addr, self.ep0, self.toggles) = (0, Ep0::default(), [false; 32]);
    }

    /// Clear `ep`'s halt as CLEAR_FEATURE(ENDPOINT_HALT) would.
    pub fn clear_halt(&mut self, ep: u8, input: bool) {
        self.toggles[2 * ep as usize + input as usize] = false;
        if input {
            self.stall_bulk_in = false;
        }
    }
}
