//! The `nox1` address the words or a key give an account, derived on the spot: no store is
//! opened and nothing is read from the network, so a wallet can show it the moment the words
//! are in hand, while the store it belongs to is still being made. It is the same address a
//! store restored from the same words answers for the same account.

use crate::custody::{parse_key, phrase_to_seed, seed_of_key, Phrase};
use crate::error::WalletError;
use crate::evm::EvmAccount;
use crate::keys::{receiving_address_text, Account};

/// The accounts of one phrase a wallet may have open, as the NONOS wallet numbers them.
const ACCOUNTS: u32 = 8;

/// The `nox1` address of the account of `words` whose public address is `account`, or None
/// when none of the phrase's first accounts is that one.
pub fn address_of_words(
    words: &[String],
    account: [u8; 20],
) -> Result<Option<String>, WalletError> {
    let seed = phrase_to_seed(&Phrase::parse(words)?)?;
    for index in 0..ACCOUNTS {
        let public = EvmAccount::at(seed.bytes(), index).map(|a| a.address());
        if public == Some(account) {
            let shield = Account::at(&seed, index)?;
            return Ok(Some(receiving_address_text(&shield.receiving_address())));
        }
    }
    Ok(None)
}

/// The `nox1` address of a wallet made from the private key `key`, or None when the key is
/// not the account `account`.
pub fn address_of_key(key: &str, account: [u8; 20]) -> Result<Option<String>, WalletError> {
    let key = parse_key(key)?;
    if EvmAccount::from_key(&key).map(|a| a.address()) != Some(account) {
        return Ok(None);
    }
    let shield = Account::at(&seed_of_key(&key), 0)?;
    Ok(Some(receiving_address_text(&shield.receiving_address())))
}
