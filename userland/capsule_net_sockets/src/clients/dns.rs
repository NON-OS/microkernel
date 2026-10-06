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

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::mk_uptime_ms;

use super::envelope::call_t;

const MAGIC: u32 = 0x4E44_4E53;
const RESOLVE_A: u16 = 2;
/// The envelope's code for a call that went unanswered.
const NO_ANSWER: u16 = 15;
/// net.dns's own answer when no upstream server replied in its 3 s.
const DNS_TIMEOUT: u16 = 6;
/// How long a lookup may hold the serve loop. net.dns answers a name it can
/// reach in well under this; with no upstream it did not answer at all, and
/// every caller of this service waited out the kernel's 5 s default, once per
/// name, which froze the browser on a Direct page for up to 9 s.
const RESOLVE_MS: u64 = 4_000;
/// After net.dns went unanswered, or said no upstream server answered it,
/// lookups are refused at once for this long instead of each waiting again.
/// A name net.dns answered as unknown does not count: the network is there.
const QUIET_MS: i64 = 5_000;

static QUIET_UNTIL: AtomicI64 = AtomicI64::new(0);

pub fn resolve_a(port: u32, host: &[u8]) -> Result<[u8; 4], u16> {
    if port == 0 {
        return Err(NO_ANSWER);
    }
    let now = mk_uptime_ms();
    if now < QUIET_UNTIL.load(Ordering::Relaxed) {
        return Err(NO_ANSWER);
    }
    let mut out = [0u8; 4];
    match call_t(port, MAGIC, RESOLVE_A, host, &mut out, RESOLVE_MS) {
        Ok(4) => Ok(out),
        Ok(_) => Err(4),
        Err(e) if e == NO_ANSWER || e == DNS_TIMEOUT => {
            QUIET_UNTIL.store(mk_uptime_ms() + QUIET_MS, Ordering::Relaxed);
            Err(e)
        }
        Err(e) => Err(e),
    }
}
