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

use super::phase::Phase;

pub(crate) const PHASES: usize = 11;
pub(crate) const NAME_MAX: usize = 32;

pub(crate) const ORDER: [Phase; PHASES] = [
    Phase::Begin,
    Phase::CertVerified,
    Phase::ManifestVerified,
    Phase::Attested,
    Phase::ProcessCreated,
    Phase::ImageLoaded,
    Phase::Installed,
    Phase::Queued,
    Phase::FirstSyscall,
    Phase::FirstSurface,
    Phase::FirstFrame,
];

/*
 * One launch: the capsule's name, its pid once it has one, and the clock at
 * each phase it reached. A phase is kept the first time it is reached, so a
 * later syscall or frame does not move the mark.
 */
#[derive(Clone, Copy)]
pub(crate) struct Record {
    name: [u8; NAME_MAX],
    name_len: u8,
    pid: u32,
    reached: u16,
    at: [u64; PHASES],
}

impl Record {
    pub(crate) fn begin(name: &[u8], now_ns: u64) -> Self {
        let mut r = Self {
            name: [0; NAME_MAX],
            name_len: 0,
            pid: 0,
            reached: 0,
            at: [0; PHASES],
        };
        let n = name.len().min(NAME_MAX);
        r.name[..n].copy_from_slice(&name[..n]);
        r.name_len = n as u8;
        r.mark(Phase::Begin, now_ns);
        r
    }

    pub(crate) fn mark(&mut self, phase: Phase, now_ns: u64) {
        let i = phase as usize;
        if self.reached & (1 << i) == 0 {
            self.at[i] = now_ns;
            self.reached |= 1 << i;
        }
    }

    pub(crate) fn name(&self) -> &[u8] {
        &self.name[..self.name_len as usize]
    }

    pub(crate) fn pid(&self) -> u32 {
        self.pid
    }

    pub(crate) fn set_pid(&mut self, pid: u32) {
        self.pid = pid;
    }

    pub(crate) fn reached(&self, phase: Phase) -> bool {
        self.reached & (1 << phase as usize) != 0
    }

    /*
     * Time from the last phase reached before this one. Phases a launch
     * skipped (a publisher-tier capsule has no attestation) fold into the
     * next one that was reached, so the spans always sum to the total.
     */
    pub(crate) fn span_ns(&self, phase: Phase) -> Option<u64> {
        let i = phase as usize;
        if i == 0 || !self.reached(phase) {
            return None;
        }
        let mut j = i;
        while j > 0 {
            j -= 1;
            if self.reached & (1 << j) != 0 {
                return Some(self.at[i].saturating_sub(self.at[j]));
            }
        }
        None
    }

    pub(crate) fn total_ns(&self) -> u64 {
        let mut last = 0;
        for (i, at) in self.at.iter().enumerate() {
            if self.reached & (1 << i) != 0 {
                last = *at;
            }
        }
        last.saturating_sub(self.at[0])
    }
}
