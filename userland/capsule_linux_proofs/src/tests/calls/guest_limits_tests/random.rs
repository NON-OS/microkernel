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

//! Random guest bytes never panic or run on.

use super::super::super::random::Regs;
use crate::linux::net::dns::{name, reply};
use crate::linux::unix::cmsg::{rights, MAX_FDS};
use crate::wire::{malformed, next};

/// Random control blocks, queries and display streams: nothing panics, the
/// walk ends, and what comes out lies within what went in.
#[test]
fn random_guest_bytes_never_panic_or_run_on() {
    let mut r = Regs::new(0x6775_6573_745f_6c69);
    for _ in 0..20_000 {
        let len = r.small(1100) as usize;
        let mut raw: Vec<u8> = (0..len).map(|_| r.any() as u8).collect();
        // Plant plausible headers so the walk goes some way in.
        for at in (0..len.saturating_sub(16)).step_by(8 + 8 * r.small(4) as usize) {
            if r.small(2) == 0 {
                raw[at..at + 8].copy_from_slice(&(16 + 4 * r.small(8)).to_le_bytes());
                raw[at + 8..at + 16].copy_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0]);
            }
        }
        let fds = rights(&raw);
        assert!(fds.len() <= MAX_FDS && fds.len() * 4 <= raw.len());
        if let Some((host, end)) = name::read(&raw) {
            assert!(host.len() <= 255 && end <= raw.len());
            if let Some(question) = raw.get(..end + 4) {
                assert!(reply::build(question, Some([1, 2, 3, 4])).len() <= question.len() + 16);
            }
        }
        let _ = malformed(&raw);
        if let Some((m, used)) = next(&raw) {
            assert!(used >= 8 && used <= raw.len() && m.args.len() == used - 8);
        }
    }
}
