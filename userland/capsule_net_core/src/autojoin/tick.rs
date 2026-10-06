//! When to try, and when to stop trying.
//!
//! Only while nothing is bound: a link that is already up is left alone. What
//! is due comes from `machine`; every call made here waits at most `CALL_MS`,
//! so the serve loop goes back to its clients within a few of those. A join
//! the driver is still running is watched on later ticks, never waited for.

use spin::Mutex;

use nonos_wifi_client::find;

use super::attempt::{attempt, forget_saved};
use super::machine::{Due, Machine, CALL_MS};

static MACHINE: Mutex<Machine> = Mutex::new(Machine::new());

pub fn tick(now: i64) {
    let mut m = MACHINE.lock();
    match m.due(now) {
        Due::Nothing => {}
        Due::Attempt if crate::setup::bound_port() != 0 => m.bound(now),
        Due::Attempt => {
            let step = attempt(m.tried());
            m.after_attempt(now, step);
            if m.done() && matches!(step, super::machine::Step::NoneInRange) {
                say(b"[NET-CORE] no saved Wi-Fi network in range; join one from Settings\n");
            }
        }
        Due::Watch => {
            let associated = find().and_then(|d| d.link_within(CALL_MS)).map(|l| l.associated);
            match m.after_watch(now, associated) {
                Some((_, true)) => say(b"[NET-CORE] joined a saved Wi-Fi network\n"),
                Some((_, false)) => say(b"[NET-CORE] saved Wi-Fi network not joined in time\n"),
                None => {}
            }
        }
    }
    // The passphrases are kept only while autojoin may still use them.
    if m.done() {
        forget_saved();
    }
}

pub(super) fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
