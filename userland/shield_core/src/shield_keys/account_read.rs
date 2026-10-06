//! What an account shows: the addresses other wallets pay, and the keys a scan opens notes with.

use super::super::address::Address;
use super::super::receive::ReceiveKey;
use super::super::receiving_address::{receiving_address, RECEIVING_ADDRESS_BYTES};
use super::super::view::ViewKey;
use super::Account;

impl Account {
    /// The address to hand to a sender.
    pub fn address(&self) -> Address {
        let mut words = [0u64; 4];
        for (slot, e) in words.iter_mut().zip(self.spend_pk.iter()) {
            *slot = e.value();
        }
        Address { spend_pk: words, view_pk: self.view.public() }
    }

    /// The 1,249-byte address other wallets pay: version, spend key, X-Wing key.
    pub fn receiving_address(&self) -> [u8; RECEIVING_ADDRESS_BYTES] {
        let spend = [
            self.spend_pk[0].value(),
            self.spend_pk[1].value(),
            self.spend_pk[2].value(),
            self.spend_pk[3].value(),
        ];
        receiving_address(&spend, &self.receive.encapsulation_key())
    }

    /// The viewing key, which can see incoming notes and never spend them.
    pub fn view(&self) -> &ViewKey {
        &self.view
    }

    /// The X-Wing receiving key, for opening notes during a scan.
    pub(crate) fn receive(&self) -> &ReceiveKey {
        &self.receive
    }
}
