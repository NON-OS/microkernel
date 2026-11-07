// NONOS Operating System (AGPL-3.0-or-later)
//! The two boot stages init closes before its report line: the bring-up
//! ledger's summary and the start of the boot census, each at the clock's
//! reading. They record what they were asked, in order, so a test reads back
//! that the report closed both, and with what.

use std::sync::Mutex;

/// The uptime the clock stand-in reads, in milliseconds.
pub const NOW_MS: u64 = 4_242;

pub static STEPS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Every stage asked so far, and the record emptied.
pub fn take() -> Vec<String> {
    core::mem::take(&mut *STEPS.lock().unwrap())
}

fn step(s: String) {
    STEPS.lock().unwrap().push(s);
}

pub mod bringup {
    pub fn summary(stage: &'static str) {
        super::step(format!("bringup summary {stage}"));
    }
}

pub mod clock {
    pub fn since_boot_ms() -> u64 {
        super::NOW_MS
    }
}

pub mod boot_census {
    pub fn start(now_ms: u64) {
        super::step(format!("census start {now_ms}"));
    }
}
