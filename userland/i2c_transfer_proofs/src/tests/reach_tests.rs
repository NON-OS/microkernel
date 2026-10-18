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

//! Which devices a client of driver.i2c_pci0 can reach. The HID driver
//! parses descriptors the device controls, so it is the capsule a hostile
//! pad attacks first; through OP_TRANSFER and OP_PROBE it must reach the pad
//! setup bound and nothing else on the bus: not the PMIC, the USB-PD
//! controller, the fuel gauge, the SPD EEPROM or an I2C TPM.

use nonos_i2cmodel::Config;
use nonos_libc::take_reply;

use super::fixture::{bench, core, PAD};
use crate::protocol::{parse, Request, E_INVAL, E_OK, IPC_PAYLOAD_MAX, OP_PROBE, OP_TRANSFER};
use crate::server::handlers::{probe, transfer};

const CALLER: u32 = 42;
/// A bus neighbour the pad's driver has no business with.
const PMIC: u8 = 0x2C;

fn errno(op: u16, run: impl FnOnce(&Request, &mut [u8])) -> i32 {
    let req = Request { op, request_id: 3 };
    let mut out = [0u8; IPC_PAYLOAD_MAX];
    run(&req, &mut out);
    let (_, bytes) = take_reply().expect("the handler replied");
    let (_, payload) = parse(&bytes).expect("reply header parses");
    i32::from_le_bytes(payload[0..4].try_into().expect("errno"))
}

fn transfer_to(b: &super::fixture::Bench, addr: u8) -> i32 {
    let body = [addr, 0, 2, 0, 2, 0, 1, 0, 0x01, 0x00];
    errno(OP_TRANSFER, |req, out| transfer::handle(&b.driver, CALLER, req, &body, out))
}

fn probe_of(b: &super::fixture::Bench, addr: u8) -> i32 {
    errno(OP_PROBE, |req, out| probe::handle(&b.driver, CALLER, req, &[addr], out))
}

fn bus_untouched() -> bool {
    core(|c| c.bus().trace().is_empty())
}

#[test]
fn a_transfer_to_any_device_but_the_bound_pad_is_refused_before_the_bus() {
    let b = bench(Config::LPSS);
    for addr in [PMIC, 0x00, 0x14, 0x16, 0x50, 0x7F, 0x80, 0x95, 0xFF] {
        assert_eq!(transfer_to(&b, addr), E_INVAL, "address {addr:#04x}");
        assert!(bus_untouched(), "address {addr:#04x} reached the bus");
    }
    assert_eq!(transfer_to(&b, PAD), E_OK, "the bound pad itself");
}

#[test]
fn a_probe_of_any_device_but_the_bound_pad_is_refused_before_the_bus() {
    let b = bench(Config::LPSS);
    for addr in [PMIC, 0x00, 0x50, 0x7F, 0x80, 0xFF] {
        assert_eq!(probe_of(&b, addr), E_INVAL, "address {addr:#04x}");
        assert!(bus_untouched(), "address {addr:#04x} reached the bus");
    }
    assert_eq!(probe_of(&b, PAD), E_OK, "the bound pad itself");
}
