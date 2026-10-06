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

//! Signal dispositions and masks, recorded here and acted on in
//! `serve::deliver`: a handler is kept with its flags, restorer and mask, and
//! each thread has the mask of signals it holds back (signal_mask). The
//! struct and the numbers it may name are in `signal_act`, which the host
//! proofs hold.
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub use super::signal_act::SIGSET_LEN;
use super::signal_act::{decode, encode, signal_of, SIGACTION_LEN};

/// In Linux's order: the size, then the new action read, then the signal it
/// names, then the old action written, once the new one is in place.
pub fn rt_sigaction(guest: &mut Guest, signum: u64, act: u64, old: u64, size: u64) -> u64 {
    if size != SIGSET_LEN {
        return errno::fail(errno::EINVAL);
    }
    let new = match act {
        0 => None,
        at => match guest.read(at, SIGACTION_LEN).as_deref().and_then(decode) {
            Some(a) => Some(a),
            None => return errno::fail(errno::EFAULT),
        },
    };
    let n = match signal_of(signum, new.is_some()) {
        Ok(n) => n,
        Err(e) => return errno::fail(e),
    };
    let was = guest.signals.action(n).unwrap_or_default();
    if let Some(a) = new {
        guest.signals.set(n, a);
        /* A signal set to be ignored is dropped where it already waits. */
        if guest.signals.discards(n as u8) {
            guest.signals.discard(n as u8);
        }
    }
    if old != 0 && guest.write(old, &encode(was)) < SIGACTION_LEN as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
