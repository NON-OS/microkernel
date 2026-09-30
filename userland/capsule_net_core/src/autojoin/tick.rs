//! When to try, and when to stop trying.
//!
//! Only while nothing is bound: a link that is already up is left alone. One
//! join is tried per call, so the serve loop answers between attempts; a join
//! still holds the loop for as long as the driver takes (up to 20 seconds),
//! during which net_core does not answer. Each saved network is tried at most
//! once per boot, so a wrong passphrase is not replayed at the access point.

use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, Ordering};

use super::attempt::{attempt, Step};

static NEXT_MS: AtomicI64 = AtomicI64::new(0);
static DONE: AtomicBool = AtomicBool::new(false);
static TRIED: AtomicU8 = AtomicU8::new(0);
/// Passes with the driver ready and no saved network in range.
static EMPTY_PASSES: AtomicU8 = AtomicU8::new(0);

/// Waiting for the driver, the store or the policy restore.
const NOT_YET_MS: i64 = 3_000;
/// Waiting for a saved network to come into range.
const RESCAN_MS: i64 = 15_000;
/// About two minutes of rescans after the driver is ready, then Settings.
const EMPTY_PASSES_MAX: u8 = 8;

pub fn tick(now: i64) {
    if DONE.load(Ordering::Relaxed) || now < NEXT_MS.load(Ordering::Relaxed) {
        return;
    }
    let wait = if crate::setup::bound_port() != 0 {
        NOT_YET_MS
    } else {
        match attempt(TRIED.load(Ordering::Relaxed)) {
            Step::NotYet => NOT_YET_MS,
            Step::Tried(mask) => {
                TRIED.store(mask, Ordering::Relaxed);
                NOT_YET_MS
            }
            Step::NoneInRange
                if EMPTY_PASSES.fetch_add(1, Ordering::Relaxed) + 1 >= EMPTY_PASSES_MAX =>
            {
                say(b"[NET-CORE] no saved Wi-Fi network in range; join one from Settings\n");
                DONE.store(true, Ordering::Relaxed);
                return;
            }
            Step::NoneInRange => RESCAN_MS,
            Step::Done => {
                DONE.store(true, Ordering::Relaxed);
                return;
            }
        }
    };
    NEXT_MS.store(now + wait, Ordering::Relaxed);
}

pub(super) fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
