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

//! What the device-tree platform finder asks of a DesignWare I2C node it
//! matched by `NEEDLES`: the controller's input clock and the bus rate.
//! The finder reads `reg`, the interrupt and `status` itself, and writes
//! the bus record; this reads only `clocks` and `clock-frequency`.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::property::Property;

use super::phandle::provider;

/// The compatibles the DesignWare I2C capsule drives.
pub const NEEDLES: &[&[u8]] = &[b"snps,designware-i2c"];
/// The bus rate when the node names none: standard mode, 100 kHz (I2C-bus
/// specification, as recalled, unverified).
pub const DEFAULT_BUS_HZ: u32 = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2cBus {
    /// The controller's input clock (ic_clk) in Hz; 0 when no fixed clock
    /// names it, which the capsule refuses rather than guess.
    pub input_hz: u32,
    /// The SCL rate the tree asks for.
    pub bus_hz: u32,
}

/// `props` are the matched node's own properties, as the finder's node
/// gives them. The input clock is the rate of the first `clocks` entry
/// when that is a fixed clock, never the node's own `clock-frequency`,
/// which is the bus rate; a rate of 0 names none.
pub fn describe<'a>(
    fdt: &Fdt<'a>,
    props: impl IntoIterator<Item = Property<'a>>,
) -> Result<I2cBus, FdtError> {
    let (mut clock, mut bus) = (None, None);
    for p in props {
        match p.name {
            b"clocks" => clock = Some(be_u32(p.data, 0)?),
            b"clock-frequency" => bus = Some(be_u32(p.data, 0)?),
            _ => {}
        }
    }
    let input_hz = match clock {
        Some(phandle) => provider(fdt, phandle)?.and_then(|p| p.fixed_hz).unwrap_or(0),
        None => 0,
    };
    let bus_hz = bus.filter(|&hz| hz != 0).unwrap_or(DEFAULT_BUS_HZ);
    Ok(I2cBus { input_hz, bus_hz })
}
