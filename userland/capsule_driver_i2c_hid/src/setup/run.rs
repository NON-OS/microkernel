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

use nonos_libc::Deadline;

use super::bind::bind;
use super::say::say;
use crate::hid::{probe_addr, sleep};
use crate::i2c_client::{query_acpi_hid, resolve};
use crate::state::State;

/// How long a pad has to come up, from the first probe (plan/50 C7): well
/// past the controller's own bring-up, which answers E_BUSY until its bus
/// is up, however long each ask takes.
pub const PROBE_WINDOW_MS: u64 = 30_000;

pub fn run() -> Result<State, &'static str> {
    let (port, pid) = resolve().ok_or("i2c-hid: missing i2c controller")?;
    let mut state = State::new(port, pid);
    state.probe_window = Deadline::after_ms(PROBE_WINDOW_MS);
    reprobe(&mut state);
    Ok(state)
}

/// Ask the controller which device it bound, and bind that one. No other
/// address is tried: the controller found it from the firmware's
/// declaration or its own read-only sweep (capsule_driver_i2c_pci
/// setup/sequence/sweep.rs), and it refuses any address it has not put in
/// reach. Once the window has passed with nothing to drive, a device that
/// answered is put back to sleep, nothing more is asked, and the driver
/// leaves.
pub fn reprobe(state: &mut State) {
    if state.done {
        return;
    }
    state.probes += 1;
    let port = state.i2c_port;
    let named = query_acpi_hid(port)
        .and_then(|(addr, reg)| probe_addr(port, addr, reg, &mut state.descriptor).map(|_| addr));
    if let Some(addr) = named {
        bind(state, addr);
    }
    if !state.found() && !state.done && state.probe_window.expired() {
        if state.addr != 0 {
            let _ = sleep(port, state.addr, &state.descriptor);
        }
        say("driver.i2c_hid: no HID-over-I2C touchpad came up on the I2C bus; if this machine has one, it is on a controller or address NONOS cannot reach\n");
        state.done = true;
    }
}
