//! The stages. Each prints what the next one, or the reader of the chain, needs.

use super::wait::{relay, settle};
use nox_shield_core::ffi::{CancelToken, Wallet};
use nox_shield_core::net::asset::Coin;

pub fn addresses(wallet: &Wallet) {
    for account in wallet.accounts().expect("accounts") {
        println!("ACCOUNT {} public {}", account.index + 1, account.public_address);
    }
}

/// The deposit from account 1: the approval first when the pool needs one, then the deposit.
pub fn shield(wallet: &Wallet) {
    wallet.select_account(0).expect("account 1");
    for _ in 0..2 {
        let review = wallet.review_shield(Coin::Nox, "5000".into()).expect("a review");
        assert!(review.refusal.is_none(), "refused: {:?}", review.refusal);
        let sent = wallet.confirm_public_send(review.id).expect("sent");
        println!("{} {}", if review.approval { "APPROVE" } else { "DEPOSIT" }, sent.hash);
        settle(wallet, &sent.hash);
        if !review.approval {
            return;
        }
    }
    panic!("still asking for an approval after one was mined");
}

/// Account 1 sends privately to account 2, and the relayer settles it.
pub fn send(wallet: &Wallet) {
    wallet.select_account(1).expect("account 2");
    let to = wallet.receiving_address().expect("nox1 of account 2");
    wallet.select_account(0).expect("account 1");
    println!("SYNC {:?}", wallet.sync_chain().expect("synced").balances);
    let ticket = wallet
        .send_private(Coin::Nox, to, "2000".into(), true, CancelToken::new())
        .expect("proved");
    println!("WEAKENED {:?}", ticket.weakened);
    relay(wallet);
}

pub fn receive(wallet: &Wallet) {
    wallet.select_account(1).expect("account 2");
    println!("RECEIVED {:?}", wallet.sync_chain().expect("synced"));
}

/// Account 2 withdraws to account 3, a public address that has never been used.
pub fn withdraw(wallet: &Wallet) {
    let target = wallet.accounts().expect("accounts")[2].public_address.clone();
    wallet.select_account(1).expect("account 2");
    println!("SYNC {:?}", wallet.sync_chain().expect("synced").balances);
    wallet.withdraw(Coin::Nox, target, "1000".into(), true, CancelToken::new()).expect("proved");
    relay(wallet);
}
