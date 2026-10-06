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
use super::pio::grant as pio_grant;
use crate::discover::Found;
use crate::init::flush_output;
use nonos_libc::mk_device_release;

/// Whether an i8042 answers behind the keyboard record.
///
/// The broker lists the keyboard record on every machine, because the i8042
/// cannot be enumerated, so the record proves nothing: a machine whose
/// keyboard and pointer are USB or i2c has the record and no controller.
/// The ports then float, and the output buffer never empties. The probe
/// claims the record, flushes, and gives the claim back whatever it found.
/// A claim or grant the broker refuses proves nothing either and counts as
/// present, so the bounded bring-up gets to try and to say why it failed.
pub fn controller_answers(dev: Found) -> bool {
    let Ok(epoch) = claim(dev.device_id) else {
        return true;
    };
    // A refused grant has already given the claim back.
    let Ok(pio) = pio_grant(dev.device_id, epoch) else {
        return true;
    };
    let answers = flush_output(pio.grant_id);
    let _ = mk_device_release(dev.device_id);
    answers
}
