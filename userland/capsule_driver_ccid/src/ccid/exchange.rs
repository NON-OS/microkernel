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
//! One command and its answer over the bulk pair. Each bulk IN transfer is
//! one message, ended by the reader's short packet (CCID 1.1, 5.2, as
//! recalled, unverified): an answer whose dwLength runs past its transfer
//! was cut short and is passed over, never joined to the next. A late
//! answer (another slot or sequence) is let go; each time extension starts
//! the wait again, a bounded number of times. A halted pipe is cleared.

use alloc::vec::Vec;

use nonos_usbnet::xhci::{E_IO, E_PIPE};
use nonos_usbnet::Bus;

use super::reply::{whole, Outcome};

/// Time extensions taken before the card is given up on.
pub const EXTENSIONS: u32 = 64;
pub const E_TIMEDOUT: i32 = -110;

/// `cmd` sent and its answer, whole, at most `max` bytes. `wait(fresh)`
/// idles a moment and says whether time is left, `fresh` from now.
pub fn exchange<B: Bus>(
    bus: &mut B,
    cmd: &[u8],
    max: usize,
    mut wait: impl FnMut(bool) -> bool,
) -> Result<Vec<u8>, i32> {
    if bus.bulk_out(cmd).map_err(|e| halted(bus, e, false))? != cmd.len() {
        return Err(E_IO);
    }
    let (slot, seq, mut buf) = (cmd[5], cmd[6], alloc::vec![0u8; max]);
    let (mut extensions, mut fresh) = (0, true);
    loop {
        if !wait(core::mem::take(&mut fresh)) {
            return Err(E_TIMEDOUT);
        }
        let got = match bus.bulk_in_poll(&mut buf) {
            Ok(Some(n)) => &buf[..n.min(max)],
            Ok(None) => continue,
            Err(e) => return Err(halted(bus, e, true)),
        };
        let Some(r) = whole(got, max)?.map(|r| (r.slot, r.seq, r.outcome)) else { continue };
        let ours = r.0 == slot && r.1 == seq;
        if ours && r.2 != Outcome::MoreTime {
            return Ok(got.to_vec());
        }
        (extensions, fresh) = (extensions + ours as u32, ours);
        if extensions > EXTENSIONS {
            return Err(E_TIMEDOUT);
        }
    }
}

/// The errno for a failed transfer, its pipe cleared when it halted.
fn halted<B: Bus>(bus: &mut B, e: i32, dir_in: bool) -> i32 {
    if e != E_PIPE {
        return e;
    }
    let _ = bus.reset_bulk(dir_in);
    E_IO
}
