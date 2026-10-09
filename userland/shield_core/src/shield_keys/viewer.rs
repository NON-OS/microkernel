//! What a scan needs of an account, and nothing that spends: `spend_pk`, the X-Wing key that opens
//! notes, and `nk` when spent notes are to be found. An account and a view key both give one.

use super::account::Account;
use super::receive::ReceiveKey;
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

pub struct Viewer<'a> {
    pub spend_pk: [u64; 4],
    pub receive: &'a ReceiveKey,
    /// Absent for an incoming view key, which cannot tell a spent note from an unspent one.
    pub nk: Option<[Fp; RATE]>,
}

impl Account {
    pub fn viewer(&self) -> Viewer<'_> {
        Viewer { spend_pk: self.address().spend_pk, receive: self.receive(), nk: Some(self.nk()) }
    }
}
