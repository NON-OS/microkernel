//! When to try, and when to stop trying.
//!
//! Only while nothing is bound: a link that is already up is left alone, but
//! once autojoin is done it is looked at, and a rejoin after it was lost (never
//! one the person left) runs whatever is bound (the Wi-Fi port stays bound). What is due comes
//! from `machine`; every call made here waits at most `CALL_MS`, so the serve
//! loop goes back to its clients within a few of those. A join the driver is
//! still running is watched on later ticks, never waited for.

use spin::Mutex;

use super::attempt::attempt;
use super::link::look;
use super::machine::{Due, Machine, Step};
use super::relink::{rejoin_pass, watch};
use super::saved::forget_saved;

static MACHINE: Mutex<Machine> = Mutex::new(Machine::new());

pub fn tick(now: i64) {
    let mut m = MACHINE.lock();
    match m.due(now) {
        Due::Nothing if m.link_due(now) => look(&mut m, now),
        Due::Nothing => {}
        Due::Attempt if !m.rejoining() && crate::setup::bound_port() != 0 => m.bound(now),
        Due::Attempt => {
            let rejoin = m.rejoining();
            let step = if rejoin { rejoin_pass() } else { attempt(m.tried(), None) };
            m.after_attempt(now, step);
            if m.done() && !matches!(step, Step::Answered { joined: true, .. }) {
                if rejoin {
                    say(b"[NET-CORE] Wi-Fi network not joined again; join it from Settings\n");
                } else if step == Step::NoneInRange {
                    say(b"[NET-CORE] no saved Wi-Fi network in range; join one from Settings\n");
                }
            }
        }
        Due::Watch => watch(&mut m, now),
    }
    // The passphrases are kept only while autojoin may still use them.
    if m.done() {
        forget_saved();
    }
}

pub(super) fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
