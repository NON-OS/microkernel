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

//! The descriptors' own fields: SPI, UART, GpioIo and 10-bit I2C.

use super::build::{gpio, i2c, serial};
use crate::serial::{gpios, serial_buses, Bus};

fn end(t: Vec<u8>) -> Vec<u8> {
    [t, vec![0x79, 0]].concat()
}

#[test]
fn spi_and_uart() {
    let spi = serial(2, 0b10, &[0x40, 0x42, 0x0F, 0, 8, 1, 0, 2, 0], "\\_SB.SPI0");
    let uart = serial(3, 0x0030, &[0x00, 0xC2, 0x01, 0, 64, 0, 32, 0, 0, 0x0C], "\\_SB.URT0");
    let got: Vec<Bus> =
        serial_buses(&end([spi, uart].concat())).into_iter().map(|c| c.bus).collect();
    let want_spi = Bus::Spi {
        speed_hz: 1_000_000,
        data_bits: 8,
        phase: 1,
        polarity: 0,
        chip_select: 2,
        three_wire: false,
        select_active_high: true,
    };
    let want_uart =
        Bus::Uart { baud: 115_200, rx_fifo: 64, tx_fifo: 32, parity: 0, flags: 0x30, lines: 0x0C };
    assert_eq!(got, [want_spi, want_uart]);
}

#[test]
fn gpio_io_and_ten_bit() {
    let t =
        end([gpio(false, 0x0002, &[3, 4], "\\_SB.GPO1"), i2c(0x2A5, 1_000_000, true, "I2C2")]
            .concat());
    let g = &gpios(&t)[0];
    assert_eq!((g.interrupt, g.edge, g.polarity, g.wake), (false, false, 2, false), "output only");
    assert_eq!((g.pins.as_slice(), g.pull, g.debounce_centi_ms), (&[3, 4][..], 1, 10));
    assert_eq!(
        serial_buses(&t)[0].bus,
        Bus::I2c { address: 0x2A5, speed_hz: 1_000_000, ten_bit: true }
    );
    assert_eq!(serial_buses(&t)[0].source, b"I2C2");
}
