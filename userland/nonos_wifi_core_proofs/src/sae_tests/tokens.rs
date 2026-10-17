// NONOS Operating System (AGPL-3.0-or-later)
//! A forger who keeps asking for anti-clogging tokens gets five commits again,
//! not one per request until the join's deadline; past that each request is
//! discarded by name and the exchange still runs.

use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeFailure, SaeState, SaeStation, SaeStep, TOKEN_ROUNDS_MAX};

use super::known::{ADDR1, ADDR2, PW};

const TOKEN_REQ: u16 = 76;

#[test]
fn token_requests_past_the_cap_are_discarded_and_the_exchange_runs_on() {
    let pwe = derive_pwe(PW, &[1u8; 14], &ADDR1, &ADDR2).unwrap();
    let mut sta = SaeStation::start(pwe, false, &[0x11; 32], &[0x22; 32]).unwrap();
    let mut sent = 0;
    for i in 0..50u8 {
        let req = [&[0x13, 0x00][..], &[i; 32]].concat();
        if let SaeStep::Send { .. } = sta.on_frame(1, TOKEN_REQ, &req) {
            sent += 1;
        }
    }
    assert_eq!(sent, TOKEN_ROUNDS_MAX, "five commits again, then no more");
    assert_eq!(TOKEN_ROUNDS_MAX, 5);
    assert_eq!(sta.state(), SaeState::Committed, "still waiting for the AP's own commit");
    assert_eq!(sta.failure(), Some(SaeFailure::TooManyTokenRequests));
}
