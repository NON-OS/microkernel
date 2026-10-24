// NONOS Operating System (AGPL-3.0-or-later)
//! The TD words, the control plan and bulk packets, pure.

use crate::facts::td::*;
use crate::sched::td::{ctrl_word, state, token, Target, TdState};
use crate::xfer::bulk::BulkEp;
use crate::xfer::packets::{advance, packets};
use crate::xfer::plan::control_plan;

const T: Target = Target { addr: 5, ep: 2, low_speed: true };

#[test]
fn a_token_carries_pid_address_endpoint_toggle_and_length_less_one() {
    let t = token(PID_IN, T, true, 64);
    assert_eq!(t, 0x69 | 5 << 8 | 2 << 15 | 1 << 19 | 63 << 21);
    assert_eq!(token(PID_OUT, T, false, 0) >> 21, 0x7FF, "no data");
}

#[test]
fn a_ready_td_is_active_with_three_tries_low_speed_and_ioc_when_last() {
    let w = ctrl_word(T, true);
    assert_eq!(w & (STATUS_ACTIVE | CTRL_LS | CTRL_IOC), STATUS_ACTIVE | CTRL_LS | CTRL_IOC);
    assert_eq!((w >> CTRL_CERR_SHIFT) & 3, 3);
    assert_eq!(ctrl_word(Target { low_speed: false, ..T }, false) & (CTRL_LS | CTRL_IOC), 0);
}

#[test]
fn a_status_word_reads_as_active_done_or_failed() {
    assert_eq!(state(STATUS_ACTIVE | 0x7FF), TdState::Active);
    assert_eq!(state(0x7FF), TdState::Done(0));
    assert_eq!(state(17), TdState::Done(18));
    assert_eq!(state(STATUS_STALLED | 3), TdState::Failed(STATUS_STALLED));
    assert_eq!(state(1 << 19 | 3), TdState::Done(4), "a NAK alone is no error");
}

#[test]
fn a_control_read_is_setup_data_from_data1_then_an_out_status() {
    let (s, n, data) = control_plan([0x80, 6, 0, 1, 0, 0, 18, 0], 8).unwrap();
    assert_eq!((n, data), (5, 3));
    let pids: Vec<u8> = s[..n].iter().map(|s| s.pid).collect();
    assert_eq!(pids, [PID_SETUP, PID_IN, PID_IN, PID_IN, PID_OUT]);
    let toggles: Vec<bool> = s[..n].iter().map(|s| s.toggle).collect();
    assert_eq!(toggles, [false, true, false, true, true]);
    assert_eq!(s[3].len, 2);
    let (s, n, _) = control_plan([0, 5, 3, 0, 0, 0, 0, 0], 64).unwrap();
    assert_eq!((n, s[1].pid, s[1].toggle), (2, PID_IN, true), "no data: an IN status");
    assert!(control_plan([0x80, 6, 0, 2, 0, 0, 0, 2], 8).is_some(), "512 at 8: 66 TDs, all");
    assert!(control_plan([0x80, 6, 0, 2, 0, 0, 1, 2], 64).is_none(), "beyond the buffer");
}

#[test]
fn bulk_packets_alternate_from_the_endpoint_toggle_and_advance_by_count() {
    let mut ep = BulkEp { t: T, mps: 64, toggle: true };
    let (s, n) = packets(&ep, false, 0x3000, 130).unwrap();
    assert_eq!(n, 3);
    assert_eq!(
        s[..n].iter().map(|s| (s.toggle, s.len)).collect::<Vec<_>>(),
        [(true, 64), (false, 64), (true, 2)]
    );
    advance(&mut ep, 2);
    assert!(!ep.toggle, "three packets flip it once overall");
    advance(&mut ep, 1);
    assert!(!ep.toggle, "two packets leave it");
    assert!(packets(&ep, true, 0, 4097).is_err());
}
