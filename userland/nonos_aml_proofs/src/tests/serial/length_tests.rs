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

//! Every length that lies, and a type NONOS does not know.

use super::build::{gpio, i2c, serial};
use crate::serial::{gpios, serial_buses, Bus};

fn end(t: Vec<u8>) -> Vec<u8> {
    [t, vec![0x79, 0]].concat()
}

#[test]
fn lying_lengths_are_skipped() {
    let good = i2c(0x10, 100_000, false, "I2C0");
    let short_i2c = serial(1, 0, &[0, 0, 0, 0, 0x10], "I2C0");
    let csi2 = serial(4, 0, &[0; 8], "CSI0");
    let t = end([short_i2c, csi2, good.clone()].concat());
    let addrs: Vec<Bus> = serial_buses(&t).into_iter().map(|c| c.bus).collect();
    assert_eq!(addrs.len(), 1, "type data too short for I2C, and CSI-2 unknown");
    let mut over = good.clone();
    over[10] = 0xFF;
    assert!(serial_buses(&end(over)).is_empty(), "type data past the descriptor");
    let mut cut = good.clone();
    cut.truncate(cut.len() - 4);
    assert!(serial_buses(&cut).is_empty(), "the descriptor past the template");
    let mut pins_past = gpio(true, 0, &[1], "G");
    pins_past[14] = 0x80;
    assert!(gpios(&end(pins_past)).is_empty(), "pin table outside the descriptor");
    assert!(
        gpios(&end(gpio(true, 0, &[1], "G")[..20].to_vec())).is_empty(),
        "shorter than its header"
    );
}
