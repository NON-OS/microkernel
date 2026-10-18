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

//! Pointing the part at the init block and starting it. INIT reads the
//! block and raises IDON; STRT, written with IDON to clear it, turns the
//! receiver and transmitter on. Both are checked against a deadline.

use nonos_libc::Deadline;

use crate::constants::csr::{
    CSR0, CSR0_IDON, CSR0_INIT, CSR0_RXON, CSR0_STRT, CSR0_TXON, CSR1, CSR2, CSR3, CSR3_MASK_ALL,
    CSR4, CSR4_DEFAULT,
};
use crate::regs::Regs;

/// Linux pcnet32 gives IDON a hundred polls; a real part takes microseconds.
const IDON_WAIT_MS: u64 = 20;
const START_WAIT_MS: u64 = 20;

pub fn run(regs: &Regs, init_block: u32) -> Result<(), &'static str> {
    regs.set_csr(CSR3, CSR3_MASK_ALL);
    regs.set_csr(CSR4, CSR4_DEFAULT);
    regs.set_csr(CSR1, init_block as u16);
    regs.set_csr(CSR2, (init_block >> 16) as u16);
    regs.set_csr(CSR0, CSR0_INIT);
    wait(regs, CSR0_IDON, IDON_WAIT_MS).map_err(|_| "init block not taken (no IDON)")?;
    regs.set_csr(CSR0, CSR0_IDON | CSR0_STRT);
    wait(regs, CSR0_RXON | CSR0_TXON, START_WAIT_MS).map_err(|_| "part did not start")
}

fn wait(regs: &Regs, bits: u16, ms: u64) -> Result<(), ()> {
    let deadline = Deadline::after_ms(ms);
    while regs.csr(CSR0) & bits != bits {
        if deadline.expired() {
            return Err(());
        }
        core::hint::spin_loop();
    }
    Ok(())
}
