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

//! The pids a pick may take, read in one pass, and the fallback to the
//! caller.

use super::band_choice::BANDS;
use super::on_cpu::held_elsewhere;
use crate::process::nonos_core::Priority;
use alloc::vec::Vec;

/*
 * Every pid of `sorted` that is Ready, not the caller and not held by
 * another CPU, with its band. The scan this replaced looked each pid up in
 * the table and took its two locks once per band: five linear lookups per
 * pid per pick, on every CPU. Now the table is read once and each pid's
 * state and priority once.
 */
pub(super) fn candidates(sorted: &[u32], current: u32) -> Vec<(u32, usize)> {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    let mut out = Vec::with_capacity(sorted.len());
    for pcb in PROCESS_TABLE.find_sorted(sorted) {
        /*
         * Skipped rather than left to the claim to refuse: a pick that always
         * lands on the same held pid would use up every claim attempt on it.
         */
        if pcb.pid == current || held_elsewhere(pcb.pid) {
            continue;
        }
        if *pcb.state.lock() != ProcessState::Ready {
            continue;
        }
        let band = band_of(*pcb.priority.lock());
        out.push((pcb.pid, band));
    }
    out
}

fn band_of(prio: Priority) -> usize {
    let band = match prio {
        Priority::RealTime => 0,
        Priority::High => 1,
        Priority::Normal => 2,
        Priority::Low => 3,
        Priority::Idle => 4,
    };
    band.min(BANDS - 1)
}

pub(super) fn select_fallback(pids: &[u32], current: u32) -> Option<u32> {
    use crate::process::nonos_core::{ProcessState, PROCESS_TABLE};
    if !pids.contains(&current) {
        return None;
    }
    PROCESS_TABLE.find_by_pid(current).and_then(|pcb| {
        if *pcb.state.lock() == ProcessState::Ready {
            Some(current)
        } else {
            None
        }
    })
}
