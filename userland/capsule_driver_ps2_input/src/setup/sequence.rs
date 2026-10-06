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
use super::claim::claim;
use super::driver::Driver;
use super::irq::bind as irq_bind;
use super::marker::marker;
use super::open_line::open_line;
use super::pio::grant as pio_grant;
use super::release::release;
use super::say_aux::{say_aux, say_dropped, say_no_aux};
use super::setup_aux::setup_aux;
use crate::discover::find_ps2_kbd;
use crate::init::{
    disable_aux, dropped, enable_keyboard, enable_mouse, flush_output, restore_keyboard,
};

pub fn run() -> Result<Driver, &'static str> {
    let dev = find_ps2_kbd().ok_or("ps2 keyboard not present in device list")?;
    let claim_epoch = claim(dev.device_id)?;
    let mut pio_grant_id = pio_grant(dev.device_id, claim_epoch)?.grant_id;
    let irq_grant_id = match irq_bind(dev, claim_epoch, pio_grant_id) {
        Ok(out) => out.grant_id,
        Err(_) => {
            pio_grant_id = pio_grant(dev.device_id, claim_epoch)?.grant_id;
            0
        }
    };
    let aux = setup_aux();
    let aux_irq_grant_id = aux.map_or(0, |a| a.irq_grant_id);
    flush_output(pio_grant_id);
    if let Err(e) = enable_keyboard(pio_grant_id) {
        // Give back both claims, and with them the port grant and both
        // lines, so the next attempt claims the controller afresh instead
        // of being refused its own leftover claim.
        release(dev.device_id, aux);
        return Err(e);
    }
    let (mouse_enabled, mouse_wheel) = if aux_irq_grant_id != 0 {
        let outcome = enable_mouse(pio_grant_id);
        say_aux(outcome);
        match outcome {
            Ok(wheel) => (true, wheel),
            Err(_) => {
                // enable_mouse turned the aux clock on before the step that
                // failed; on a machine with no PS/2 mouse behind the port
                // (firmware-disabled aux) leaving it enabled streams garbage
                // the drain would have to discard forever. Turn it back off.
                disable_aux(pio_grant_id);
                restore_keyboard(pio_grant_id);
                (false, false)
            }
        }
    } else {
        say_no_aux();
        (false, false)
    };
    say_dropped(dropped());
    open_line(irq_grant_id);
    open_line(aux_irq_grant_id);
    marker(b"[driver_ps2] endpoint driver.ps2_kbd0 ready\n");
    Ok(Driver { pio_grant_id, irq_grant_id, aux_irq_grant_id, mouse_enabled, mouse_wheel })
}
