// NONOS Operating System (AGPL-3.0-or-later)

//! The shared station's join loop ends a step the access point never answers
//! at the step's own deadline, well inside the join's budget, as a timeout
//! naming the step.

use std::cell::Cell;

use crate::mlme::sim::{beacon, join, rsne, AP};
use nonos_wifi_core::mlme::{MlmeFailure, MlmeState, AUTH_MS};
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::softmac::control::result::{failure_code, CODE_TIMED_OUT};
use nonos_wifi_core::softmac::join::{drive, Outcome, JOIN_BUDGET_MS};
use nonos_wifi_core::softmac::radio::{Env, Radio, RxInfo};

// A radio the access point never answers: it hears nothing back.
struct Silent(usize);

impl Radio for Silent {
    fn mac(&self) -> [u8; 6] {
        [0x02, 0, 0, 0, 0, 2]
    }
    fn set_mac(&mut self, _: [u8; 6]) -> bool {
        true
    }
    fn bands(&self) -> (bool, bool) {
        (true, false)
    }
    fn tune(&mut self, _: Band, _: u8) -> bool {
        true
    }
    fn send(&mut self, _: &[u8]) -> bool {
        self.0 += 1;
        true
    }
    fn recv(&mut self, _: &mut [u8]) -> Option<RxInfo> {
        None
    }
    fn set_bss(&mut self, _: Option<[u8; 6]>) {}
}

// A clock fifty milliseconds on at every reading.
struct Clock(Cell<u64>);

impl Env for Clock {
    fn now_ms(&self) -> u64 {
        self.0.set(self.0.get() + 50);
        self.0.get()
    }
    fn random(&mut self, out: &mut [u8]) -> bool {
        out.fill(0x5A);
        true
    }
}

#[test]
fn a_silent_access_point_ends_the_join_at_the_steps_deadline() {
    let (mut radio, clock) = (Silent(0), Clock(Cell::new(0)));
    let m = join(JoinPolicy::ANY);
    let report = drive(&mut radio, &clock, m, &beacon(AP, &[&rsne(&[2], 0, 4)]), JOIN_BUDGET_MS, 0);
    assert!(matches!(report.outcome, Outcome::TimedOut));
    assert_eq!(report.failure, Some(MlmeFailure::Timeout(MlmeState::Authenticating)));
    let spent = clock.0.get();
    assert!((AUTH_MS..AUTH_MS + 200).contains(&spent), "{spent} ms, not the budget's");
    assert!(radio.0 >= 3, "the request resent inside the step's time");
    let code = failure_code(MlmeFailure::Timeout(MlmeState::Associating));
    assert_eq!(code, (CODE_TIMED_OUT, 0), "the code silence always had");
}
