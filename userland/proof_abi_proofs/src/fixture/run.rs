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

//! One family's run as the kernel would seal it: a histogram, a salt, the
//! measurements it verified, and the record over them, for an app whose set
//! allows read, write and exit_group.

use nonos_proof_abi::{hist_commit, meas_chain, syscall_root, Record, HIST_WORDS};

use super::policy::{app_id, policy, PROGRAM};

pub struct Family {
    pub hist: [u32; HIST_WORDS],
    pub salt: [u64; 4],
    pub meas: Vec<[u8; 32]>,
    pub policy: [u8; 256],
    pub record: Record,
    /// Hc, A, N and U, the isolation statement's words.
    pub publics: [u64; 10],
}

const ALLOWED: [usize; 3] = [0, 1, 231];

pub fn family() -> Family {
    let mut hist = [0u32; HIST_WORDS];
    hist[0] = 40; // read, succeeded
    hist[2 + 1] = 3; // write, failed
    hist[2 * 231] = 1; // exit_group
    hist[2 * 101 + 1] = 2; // ptrace, outside the set, refused
    hist[2 * 512 + 1] = 1; // a number past 511, refused
    let salt = [11, 22, 33, 44];
    let meas = vec![[0x11; 32], PROGRAM];
    let policy = policy(&ALLOWED);
    let hc = hist_commit(&hist, &salt).expect("salt below p");
    let mut set = [0u8; 65];
    set.copy_from_slice(&policy[128..193]);
    let a = syscall_root(&set).expect("a set within 512");
    let record = Record {
        sup_meas: [0x5C; 32],
        sup_caps: 0x3_0400_1979,
        authority: 0,
        seal_no: 7,
        trace: [0x7E; 32],
        rows: 47,
        hc,
        meas: meas_chain(&meas),
        n_meas: meas.len() as u32,
        app_id: app_id(&policy),
        diag: [0; 32],
    };
    let p = [hc[0], hc[1], hc[2], hc[3], a[0], a[1], a[2], a[3], 47, 3];
    Family { hist, salt, meas, policy, record, publics: p }
}
