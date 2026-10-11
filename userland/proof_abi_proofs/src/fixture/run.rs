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
//! measurements it verified, and the final record over them, sealed by the
//! pinned vendor supervisor, for an app whose set allows read, write, exit
//! and exit_group.

use nonos_proof_abi::{hist_commit, meas_chain, syscall_root, Record, FLAG_FINAL, HIST_WORDS};

use super::device::LISTING;
use super::policy::{app_id, policy, PROGRAM};

pub const SUP_MEAS: [u8; 32] = [0x5C; 32];
pub const SUP_CAPS: u64 = nonos_proof_abi::trust::LINUX_CAPS;

pub struct Family {
    pub hist: [u32; HIST_WORDS],
    pub salt: [u64; 4],
    pub meas: Vec<[u8; 32]>,
    pub policy: [u8; 256],
    pub record: Record,
    /// Hc and A, the isolation statement's words.
    pub publics: [u64; 8],
}

pub fn family() -> Family {
    let mut hist = [0u32; HIST_WORDS];
    hist[0] = 40; // read, through
    hist[2 + 1] = 3; // write, refused
    hist[2 * 231] = 1; // exit_group
    hist[2 * 101 + 1] = 2; // ptrace, outside the set, refused
    hist[2 * 512 + 1] = 1; // a native number, refused by the kernel
    let salt = [11, 22, 33, 44];
    let meas = vec![[0x11; 32], PROGRAM];
    let policy = policy(&LISTING, &[0, 1, 60, 231]);
    let hc = hist_commit(&hist, &salt).expect("salt below p");
    let a = syscall_root(policy[152..217].try_into().unwrap()).expect("a set within 512");
    let record = Record {
        flags: FLAG_FINAL,
        sup_meas: SUP_MEAS,
        sup_caps: SUP_CAPS,
        authority: 0,
        seal_no: 0x7A11_5EA1,
        trace: [0x7E; 32],
        rows: 47,
        hc,
        meas: meas_chain(&meas),
        n_meas: meas.len() as u32,
        app_id: app_id(&policy),
    };
    let p = [hc[0], hc[1], hc[2], hc[3], a[0], a[1], a[2], a[3]];
    Family { hist, salt, meas, policy, record, publics: p }
}
