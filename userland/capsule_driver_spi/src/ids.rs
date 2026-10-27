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

//! The Intel LPSS SPI functions this driver takes: the SPI entries of Linux
//! v6.12's intel-lpss-pci.c (spt, bxt, cnl and tgl spi_info, Sunrise Point
//! on), whose private block sits at 0x200 as facts/lpss.rs has it. The
//! older hosts spi-pxa2xx-pci.c lists (Baytrail, Braswell, Lynx Point,
//! Quark) keep it elsewhere and are not taken. No other function is ever
//! claimed, reset or has its chip select written: an LPSS I2C or UART, or
//! a signal-processing function such as a thermal sensor, shares the
//! classes. The kernel spawns for the same list (inventory/spi_host.rs).

const INTEL: u16 = 0x8086;

#[rustfmt::skip]
const SPI_IDS: [u16; 86] = [
    0x02aa, 0x02ab, 0x02fb, 0x06aa, 0x06ab, 0x06fb, 0x0ac2, 0x0ac4, 0x0ac6, 0x1ac2,
    0x1ac4, 0x1ac6, 0x31c2, 0x31c4, 0x31c6, 0x34aa, 0x34ab, 0x34fb, 0x43aa, 0x43ab,
    0x43fb, 0x43fd, 0x4b2a, 0x4b2b, 0x4b37, 0x4daa, 0x4dab, 0x4dfb, 0x51aa, 0x51ab,
    0x51fb, 0x54aa, 0x54ab, 0x54fb, 0x5ac2, 0x5ac4, 0x5ac6, 0x7727, 0x7730, 0x7746,
    0x7a2a, 0x7a2b, 0x7a79, 0x7a7b, 0x7aaa, 0x7aab, 0x7af9, 0x7afb, 0x7e27, 0x7e30,
    0x7e46, 0x7f2a, 0x7f2b, 0x7f5e, 0x7f5f, 0x98aa, 0x9d29, 0x9d2a, 0x9daa, 0x9dab,
    0x9dfb, 0xa0aa, 0xa0ab, 0xa0de, 0xa0df, 0xa0fb, 0xa0fd, 0xa0fe, 0xa129, 0xa12a,
    0xa2a9, 0xa2aa, 0xa32a, 0xa32b, 0xa37b, 0xa3a9, 0xa3aa, 0xa827, 0xa830, 0xa846,
    0xe327, 0xe330, 0xe346, 0xe427, 0xe430, 0xe446,
];

/// Whether `vendor:device` is a listed LPSS SPI host.
pub fn spi_host(vendor: u16, device: u16) -> bool {
    vendor == INTEL && SPI_IDS.contains(&device)
}
