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

use super::await_reset::await_reset;
use super::command_register::command_register;
use super::settle::{POWER_ON_MS, RETRY_MS};
use crate::i2c_client::write_read;
use nonos_libc::mk_idle_ms;

const OPCODE_RESET: u8 = 0x01;
const OPCODE_SET_POWER: u8 = 0x08;
const POWER_ON: u8 = 0x00;

/// The I2C-HID power-up sequence. Wire framing per the HID-over-I2C spec: after
/// the two command-register bytes, the THIRD byte carries the report id (or
/// power state) in its low nibble and the report type in the high nibble, and
/// the OPCODE goes in the FOURTH byte: RESET is [reg, 0x00, 0x01] and
/// SET_POWER(ON) is [reg, 0x00, 0x08]. Order follows the spec and Linux
/// i2c-hid: power on first, wait (ELAN and Goodix pads need it before they
/// accept commands), then RESET and wait for the reset sentinel. SET_IDLE is not sent:
/// its short 4-byte form is nonstandard and Linux omits it for touchpads.
/// Returns false when the command register is absent or a write is rejected;
/// the caller keeps polling regardless, which still serves devices that wake
/// without the handshake.
pub fn wake(port: u32, addr: u8, desc: &[u8; 30], input_reg: u16) -> bool {
    let cmd_reg = command_register(desc);
    if cmd_reg == 0 {
        return false;
    }
    let reg = cmd_reg.to_le_bytes();
    let set_power = [reg[0], reg[1], POWER_ON, OPCODE_SET_POWER];
    // Linux i2c_hid_set_power: some STM-based and Weida Tech parts need
    // about 400 us after the first clock edge to wake and NACK the first
    // power-on, so it is sent once more after a short pause. Then 60 ms,
    // what Goodix's Windows driver waits and several pads need before they
    // take RESET.
    if write_read(port, addr, &set_power, &mut []).is_none() {
        let _ = mk_idle_ms(RETRY_MS);
        if write_read(port, addr, &set_power, &mut []).is_none() {
            say("[i2chid] SET_POWER(ON) NACKed twice, pad not reset\n");
            return false;
        }
        say("[i2chid] SET_POWER(ON) NACKed once, sent again and taken\n");
    }
    let _ = mk_idle_ms(POWER_ON_MS);
    let reset = [reg[0], reg[1], 0x00, OPCODE_RESET];
    if write_read(port, addr, &reset, &mut []).is_none() {
        return false;
    }
    // The device answers RESET with a zero-length input report on the input
    // register. Wait for that sentinel with a bounded, yield-spaced retry loop
    // and consume it so it is not parsed as input.
    await_reset(port, addr, input_reg)
}

fn say(msg: &str) {
    crate::diag::line(alloc::string::String::from(msg));
}
