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

//! A family's sealed histogram for an app whose set allows read, write, exit
//! and exit_group: forty reads, three refused writes, a refused ptrace
//! outside the set, and an exit_group. Proven once per test run.

use std::sync::OnceLock;

use isolation_stark::{prove, Statement, Witness};
use nonos_proof_abi::HIST_WORDS;

pub const ALLOWED: [usize; 4] = [0, 1, 60, 231];

pub fn witness() -> Witness {
    let mut hist = [0u32; HIST_WORDS];
    hist[0] = 40;
    hist[2 + 1] = 3;
    hist[2 * 231] = 1;
    hist[2 * 101 + 1] = 2;
    hist[2 * 512 + 1] = 1;
    let mut set = [0u8; 65];
    ALLOWED.iter().for_each(|&b| set[b / 8] |= 1 << (b % 8));
    Witness { hist, salt: [11, 22, 33, 44], set }
}

pub fn proven() -> &'static (Statement, Vec<u8>) {
    static ONE: OnceLock<(Statement, Vec<u8>)> = OnceLock::new();
    ONE.get_or_init(|| prove(&witness(), &[0x5A; 64]).expect("a proof"))
}
