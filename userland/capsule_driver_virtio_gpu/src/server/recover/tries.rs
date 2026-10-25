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
//! The tries one stall gets, each logged, and the state they end in.

use nonos_gfx_serve::Platform;
use nonos_libc::mk_debug;

use super::restore::attempt;
use crate::clock::now_ms;
use crate::driver::Driver;
use crate::state::Health;

const TRIES: [&[u8]; 3] = [
    b"driver.virtio_gpu0: control queue stalled; device reset, recovering (try 1 of 3)\n",
    b"driver.virtio_gpu0: control queue stalled; device reset, recovering (try 2 of 3)\n",
    b"driver.virtio_gpu0: control queue stalled; device reset, recovering (try 3 of 3)\n",
];
const RECOVERED: &[u8] = b"driver.virtio_gpu0: recovered in place, same surfaces\n";
const DOWN: &[u8] = b"driver.virtio_gpu0: down; device reset, claim and surfaces kept\n";
pub const IN_A_ROW_DOWN: &[u8] =
    b"driver.virtio_gpu0: down; stalled after each recovery in a row, claim and surfaces kept\n";

pub fn recover<P: Platform>(driver: &mut Driver<P>) {
    for say in TRIES {
        log(say);
        if attempt(driver).is_ok() {
            let t = &mut driver.tally;
            t.recoveries = t.recoveries.saturating_add(1);
            t.in_a_row = t.in_a_row.saturating_add(1);
            t.recovered_at = driver.server.display.q.published();
            t.recovered_ms = now_ms();
            return log(RECOVERED);
        }
        driver.server.display.q.reset_device();
    }
    down(driver, DOWN);
}

/// Left reset, with the claim and every grant kept; the outputs LOST from
/// now on, under a generation of their own so the compositor looks again.
pub fn down<P: Platform>(driver: &mut Driver<P>, say: &[u8]) {
    let gpu = &mut driver.server.display;
    gpu.health = Health::Down;
    gpu.generation = gpu.generation.wrapping_add(1);
    log(say);
}

fn log(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
