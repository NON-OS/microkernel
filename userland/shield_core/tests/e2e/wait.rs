//! Waiting on the chain and on the relayer, with every step printed for the reader of the run.

use nox_shield_core::evm::{Network, SendStatus};
use nox_shield_core::ffi::Wallet;
use std::time::Duration;

pub fn relay(wallet: &Wallet) {
    let handed = wallet.hand_to_relayer().expect("handed");
    let id = handed.id.unwrap_or_else(|| panic!("refused: {:?}", handed.refusal));
    for _ in 0..60 {
        let state = wallet.relayer_state(id.clone()).expect("state");
        println!("RELAY {} {:?}", state.status, state.tx);
        if state.status == "settled" || state.status == "refused" {
            return;
        }
        std::thread::sleep(Duration::from_secs(20));
    }
    panic!("not settled in 20 minutes");
}

pub fn settle(wallet: &Wallet, hash: &str) {
    for _ in 0..40 {
        let status = wallet.public_send_status(Network::Sepolia, hash.into()).expect("status");
        println!("STATUS {status:?}");
        match status {
            SendStatus::Confirmed => return,
            SendStatus::Failed => panic!("{hash} failed on chain"),
            SendStatus::Pending => {}
        }
        std::thread::sleep(Duration::from_secs(15));
    }
    panic!("not mined in 10 minutes");
}
