//! Everything one account of the seed holds, derived on unlock and dropped on lock.
//! The spend secret never crosses the FFI: crate internal, silent Debug, zeroed on drop.

use super::derive::field_key;
use super::domain::SPEND_CONTEXT;
use super::material::Material;
use super::receive::ReceiveKey;
use super::view::ViewKey;
use crate::custody::Seed;
use crate::error::CustodyError;
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use stark_proofs::shield::key::derive;
use stark_proofs::shield::note::POOL_LOG_ROUNDS;

/// The spend secret, the viewing key and the address publishing both public halves.
pub struct Account {
    pub(super) sk: [Fp; RATE],
    spend_pk: [Fp; RATE],
    nk: [Fp; RATE],
    view: ViewKey,
    receive: ReceiveKey,
}

impl Account {
    /// Account 0, whose keys come from the seed alone.
    pub fn from_seed(seed: &Seed) -> Result<Account, CustodyError> {
        Account::at(seed, 0)
    }

    /// Account `index` of the seed. No key is ever written to storage.
    pub fn at(seed: &Seed, index: u32) -> Result<Account, CustodyError> {
        let material = Material::of(seed, index);
        let seed = material.bytes();
        let sk = field_key(seed, SPEND_CONTEXT)?;
        let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
        let keys = derive(&h, sk);
        Ok(Account {
            sk,
            spend_pk: keys.spend_pk,
            nk: keys.nk,
            view: ViewKey::from_seed(seed),
            receive: ReceiveKey::from_seed(seed),
        })
    }

    /// The spend secret, for the witness. Crate internal, so no shell can read it.
    pub(crate) fn sk(&self) -> [Fp; RATE] {
        self.sk
    }

    /// The committed spend key the notes this account owns carry.
    pub(crate) fn spend_pk(&self) -> [Fp; RATE] {
        self.spend_pk
    }

    /// The nullifier key. Crate internal, it never crosses the shell boundary.
    pub(crate) fn nk(&self) -> [Fp; RATE] {
        self.nk
    }
}

#[path = "account_read.rs"]
mod account_read;
