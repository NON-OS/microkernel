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

//! The guest's threads, and the ones parked on a futex.

use alloc::vec::Vec;

use nonos_libc::mk_foreign_reply;

use super::fd::Kind;
use super::futex_pick::{pick, MATCH_ANY};
use super::handle::Guest;

impl Guest {
    /// The net.sockets handle behind `fd`, if it is a socket.
    pub fn socket_handle(&self, fd: u64) -> Option<u32> {
        match self.fds.get(fd as usize) {
            Some(f) if f.kind == Kind::Socket => Some(f.handle),
            _ => None,
        }
    }

    /// True for the guest and for every thread of it, which is what the
    /// serve loop needs: a trap arrives under the thread's own tid.
    pub fn owns(&self, pid: u32) -> bool {
        pid == self.pid || self.threads.contains(&pid)
    }

    /// Reply to at most `count` waiters on `uaddr` and report how many.
    /// The reply is the wake: each was left parked inside its own trap.
    pub fn wake(&mut self, uaddr: u64, count: u64) -> u64 {
        self.wake_bits(uaddr, count, MATCH_ANY)
    }

    /// `wake`, taking only waiters whose bitset meets `bits` (futex_pick).
    /// One whose reply fails has gone and does not count.
    pub fn wake_bits(&mut self, uaddr: u64, count: u64, bits: u32) -> u64 {
        let matching = pick(&self.waits, |t| self.futex_bits_of(t), uaddr, bits, u64::MAX);
        let mut taken = Vec::new();
        let mut woken = 0;
        for at in matching {
            if woken >= count {
                break;
            }
            let tid = self.waits[at].0;
            taken.push(tid);
            if mk_foreign_reply(tid, 0) >= 0 {
                woken += 1;
                self.rearm(tid);
            }
        }
        for tid in taken {
            self.forget_futex(tid);
        }
        woken
    }

    /// The bitset `tid` waits with: MATCH_ANY unless WAIT_BITSET named one.
    pub fn futex_bits_of(&self, tid: u32) -> u32 {
        self.futex_bits.iter().find(|(t, _)| *t == tid).map_or(MATCH_ANY, |&(_, b)| b)
    }

    /// Take `tid` out of its futex wait, its timeout and its bitset.
    pub fn forget_futex(&mut self, tid: u32) {
        self.waits.retain(|&(w, _)| w != tid);
        self.futex_until.retain(|&(_, t)| t != tid);
        self.futex_bits.retain(|&(t, _)| t != tid);
    }

    /// Drop every wait `tid` is parked in, for a thread that is being ended:
    /// a wait left behind could later take what a live thread waits for.
    pub fn forget_waits(&mut self, tid: u32) {
        self.forget_futex(tid);
        self.blocked.retain(|w| w.tid != tid);
    }
}
