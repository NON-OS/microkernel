// NONOS Operating System (AGPL-3.0-or-later)
//! Standing a part down waits for its bus master to stop before any grant
//! goes back: at once when it has stopped, the full wait when it has not.

use std::time::Duration;

use crate::bring::stand_down::{run, MASTER_STOP_MS};
use crate::facts::mac::CTRL_GIO_DIS;
use crate::facts::regs::{REG_CTRL, REG_STATUS};

use super::model::{regs, timed, window};

#[test]
fn a_master_that_stops_is_not_waited_on() {
    let bar = window();
    bar.present32(REG_STATUS, 0);
    let (stopped, took) = timed(|| run(&regs(&bar)));
    assert!(stopped);
    assert!(took < Duration::from_millis(MASTER_STOP_MS / 2));
    assert_ne!(bar.wrote32(REG_CTRL) & CTRL_GIO_DIS, 0, "asked to stop");
}

#[test]
fn a_master_still_running_is_given_its_whole_wait() {
    let bar = window();
    bar.present32(REG_STATUS, 1 << 19);
    let (stopped, took) = timed(|| run(&regs(&bar)));
    assert!(!stopped);
    assert!(took >= Duration::from_millis(MASTER_STOP_MS), "waited {took:?}");
}
