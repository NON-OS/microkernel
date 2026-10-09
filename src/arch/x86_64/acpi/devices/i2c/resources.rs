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

//! The resource fields the address/pin extractor (aml::crs) does not carry:
//! the I2C ConnectionSpeed and addressing mode, the GpioInt trigger and
//! polarity, and an Interrupt (Extended IRQ) resource for devices wired to
//! an APIC line instead of a GPIO. Each value is taken from the descriptor
//! that matches what the extractor already chose (same slave address, same
//! pin), so a multi-template body cannot pair the address of one fragment
//! with the speed of another. Layouts per ACPI 6.4 sections 6.4.3.6,
//! 6.4.3.8.1 and 6.4.3.8.2.1.

const LEAD_EXT_IRQ: u8 = 0x89;
const LEAD_GPIO: u8 = 0x8C;
const LEAD_SERIAL_BUS: u8 = 0x8E;
const SERIAL_BUS_I2C: u8 = 1;
const GPIO_INT: u8 = 0;
const MAX_STEPS: usize = 1 << 20;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(super) struct Extras {
    pub speed_hz: u32,
    pub ten_bit: bool,
    /// GpioInt InterruptAndIoFlags of the descriptor carrying the pin.
    pub gpio_flags: Option<u16>,
    /// First Extended IRQ: (GSI, vector flags).
    pub apic: Option<(u32, u8)>,
}

/// Call `f(lead, data)` for every byte offset that holds a large descriptor
/// whose declared length fits the region. Bounded.
fn each_large(region: &[u8], mut f: impl FnMut(u8, &[u8])) {
    let mut i = 0usize;
    while i + 3 <= region.len() && i < MAX_STEPS {
        let lead = region[i];
        if matches!(lead, LEAD_EXT_IRQ | LEAD_GPIO | LEAD_SERIAL_BUS) {
            let len = usize::from(u16::from_le_bytes([region[i + 1], region[i + 2]]));
            if let Some(data) = region.get(i + 3..i + 3 + len) {
                f(lead, data);
            }
        }
        i += 1;
    }
}

/// Fill what `regions` (searched in order) say about the descriptors that
/// carry `slave_addr` and `gpio_pin`. A field keeps the first value found.
pub(super) fn extras(regions: &[&[u8]], slave_addr: u8, gpio_pin: Option<u16>) -> Extras {
    let mut ex = Extras::default();
    let mut have_bus = false;
    for region in regions {
        each_large(region, |lead, d| match lead {
            LEAD_SERIAL_BUS if !have_bus && d.len() >= 15 && d[2] == SERIAL_BUS_I2C => {
                let raw = u16::from_le_bytes([d[13], d[14]]);
                if slave_addr != 0 && (raw & 0x7F) as u8 == slave_addr {
                    ex.speed_hz = u32::from_le_bytes([d[9], d[10], d[11], d[12]]);
                    // TypeSpecificFlags bit 0: 10-bit slave addressing.
                    ex.ten_bit = d[4] & 1 != 0;
                    have_bus = true;
                }
            }
            LEAD_GPIO if ex.gpio_flags.is_none() && d.len() >= 13 && d[1] == GPIO_INT => {
                let table = usize::from(u16::from_le_bytes([d[11], d[12]]));
                let pin = table
                    .checked_sub(3)
                    .and_then(|at| d.get(at..at + 2))
                    .map(|p| u16::from_le_bytes([p[0], p[1]]));
                if pin.is_some() && pin == gpio_pin {
                    ex.gpio_flags = Some(u16::from_le_bytes([d[4], d[5]]));
                }
            }
            LEAD_EXT_IRQ if ex.apic.is_none() && d.len() >= 6 && d[1] != 0 => {
                ex.apic = Some((u32::from_le_bytes([d[2], d[3], d[4], d[5]]), d[0]));
            }
            _ => {}
        });
    }
    ex
}

/// GpioInt InterruptAndIoFlags: bit 0 is the mode (1 edge, 0 level), bits
/// 2:1 the polarity (0 active high, 1 active low, 2 both edges).
pub(super) fn gpio_level_and_high(flags: u16) -> (bool, bool) {
    (flags & 1 == 0, (flags >> 1) & 3 == 0)
}

/// Extended IRQ vector flags: bit 1 is the mode (1 edge), bit 2 the polarity
/// (1 active low).
pub(super) fn apic_level_and_high(flags: u8) -> (bool, bool) {
    (flags & 2 == 0, flags & 4 == 0)
}
