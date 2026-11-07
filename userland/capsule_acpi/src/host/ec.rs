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

//! EmbeddedControl fields: one byte per access (the interpreter reads EC
//! fields by bytes), through the EC's ports, inside the Global Lock when
//! the EC's _GLK asks for it; none once the EC has stalled or flooded. The
//! bring-up trace says each transaction and how it ended (`ec_count`).

use super::ec_count::Counted;
use super::lock_hold::{give, take};
use super::spaces::ec_ok;
use super::state::CapsuleHost;
use nonos_aml::ec;

fn locked<T>(h: &mut CapsuleHost, glk: bool, f: impl FnOnce() -> Option<T>) -> Option<T> {
    let mine = glk && !h.lock_held;
    if mine && !take(h) {
        return None;
    }
    let r = f();
    if mine {
        give(h);
    }
    r
}

pub(super) fn read(h: &mut CapsuleHost, addr: u64, width: u32) -> Option<u64> {
    let Some((ports, glk)) = h.ec.filter(|_| h.ec_health.usable()) else {
        refused(b"rd", addr, h);
        return None;
    };
    let addr = u8::try_from(addr).ok().filter(|_| ec_ok(addr, width))?;
    let mut io = Counted::new(ports);
    let Some(r) = locked(h, glk, || Some(ec::read(&ports, &mut io, addr))) else {
        say(b"rd", addr, Err(b"lock not taken"), &io, false, h);
        return None;
    };
    heed(h, &r);
    say(
        b"rd",
        addr,
        r.as_ref().map(|v| u64::from(*v)).map_err(|e| why(e, &io, false)),
        &io,
        r.is_err(),
        h,
    );
    r.ok().map(u64::from)
}

pub(super) fn write(h: &mut CapsuleHost, addr: u64, width: u32, v: u64) -> bool {
    let Some((ports, glk)) = h.ec.filter(|_| h.ec_health.usable()) else {
        refused(b"wr", addr, h);
        return false;
    };
    let Some(addr) = u8::try_from(addr).ok().filter(|_| ec_ok(addr, width)) else { return false };
    let mut io = Counted::new(ports);
    let Some(r) = locked(h, glk, || Some(ec::write(&ports, &mut io, addr, v as u8))) else {
        say(b"wr", addr, Err(b"lock not taken"), &io, false, h);
        return false;
    };
    heed(h, &r);
    say(
        b"wr",
        addr,
        r.as_ref().map(|_| v & 0xFF).map_err(|e| why(e, &io, true)),
        &io,
        r.is_err(),
        h,
    );
    r.is_ok()
}

/// One access's answer counted; the line said once if it turned the EC off.
fn heed<T>(h: &mut CapsuleHost, r: &Result<T, ec::EcError>) {
    if h.ec_health.note(r) {
        crate::ec::say_off(h.ec_health.off);
    }
}

fn why(e: &ec::EcError, io: &Counted, write: bool) -> &'static [u8] {
    match e {
        ec::EcError::Timeout if io.writes == 0 && io.stale > 0 => {
            b"timeout, output buffer never emptied"
        }
        ec::EcError::Timeout => io.stopped(write),
        ec::EcError::Io => b"port refused",
    }
}

/// `ec <rd|wr> <addr> <ok value|timeout where|...> stale=<n> polls=<n>
/// strikes=<n>`: every transaction for the trace's first minutes, a failed
/// one at any time.
fn say(op: &[u8], addr: u8, r: Result<u64, &[u8]>, io: &Counted, failed: bool, h: &CapsuleHost) {
    if !failed && crate::sys::clock::now_ms() > crate::trace::EVERY_EC_MS {
        return;
    }
    let Some(l) = crate::trace::line() else { return };
    let l = l.str(b"ec ").str(op).str(b" ").hex(u64::from(addr));
    let l = match r {
        Ok(v) => l.str(b" ok ").hex(v),
        Err(w) => l.str(b" failed ").str(w),
    };
    let l = l.str(b" stale=").dec(u64::from(io.stale)).str(b" polls=");
    l.dec(u64::from(io.status_reads)).str(b" strikes=").dec(u64::from(h.ec_health.strikes())).end();
}

fn refused(op: &[u8], addr: u64, h: &CapsuleHost) {
    if let Some(l) = crate::trace::line() {
        let l = l.str(b"ec ").str(op).str(b" ").hex(addr).str(b" refused: ");
        l.str(if h.ec.is_none() { b"no EC" } else { b"the EC is off" }).end();
    }
}
