//! The yes to a SHIELD_FROM_FRESH review (v1.4.56): its approval, or its
//! deposits in order, signed by the fresh address's own key. The checks run
//! again first; the key is derived for these sends and dropped, zeroed, with
//! the seed; each deposit's note is kept before the deposit leaves.

use super::super::ticket::approve_calldata;
use super::super::Wallet;
use super::confirm::ShieldSplitSent;
use super::fresh_part::part_ends;
use crate::custody::Vault;
use crate::error::WalletError;
use crate::evm::tx::Eip1559;
use crate::evm::{self, EvmAccount, Network};
use crate::ffi::account_types::PublicSent;
use crate::ffi::evm::to_hex;
use crate::net::asset::Coin;
use crate::store::Row;
use crate::wallet::fresh::{handed_index, shield_checked};

impl Wallet {
    /// Sign and send what the review `id` holds, from its fresh address.
    pub fn confirm_shield_from_fresh(&self, id: u64) -> Result<ShieldSplitSent, WalletError> {
        let (from, steps) = self.take_fresh(id)?;
        let index = handed_index(&self.paths().account, &from).ok_or(WalletError::ReviewExpired)?;
        // Any value makes it an ETH batch, which shield_checked then holds to ETH's rules.
        let eth = steps.iter().any(|s| s.0.value > 0);
        let (pool, token) = part_ends(if eth { Coin::Eth } else { Coin::Nox })?;
        let approve = match steps.as_slice() {
            [(tx, None)] if Some(tx.to) == token => approved(&tx.data)?,
            _ => None,
        };
        let txs: Vec<Eip1559> = steps.iter().map(|s| s.0.clone()).collect();
        let value = txs.iter().try_fold(0u128, |t, tx| t.checked_add(tx.value));
        let value = value.ok_or(WalletError::ReviewExpired)?;
        shield_checked(&txs, (pool, token), approve.as_deref(), value)
            .map_err(|_| WalletError::ReviewExpired)?;
        let seed = Vault::at(self.paths().vault.clone()).load(self.guard())?;
        let account = EvmAccount::at(seed.bytes(), index).filter(|a| a.address() == from);
        drop(seed);
        let key = account.and_then(|a| a.signing_key()).ok_or(WalletError::ReviewExpired)?;
        let tor = self.tor()?;
        let mut sent = Vec::with_capacity(steps.len());
        for (tx, note) in steps {
            if let Some(note) = note {
                self.with_mut(|s| Ok(s.record(Row::Deposit(note))?))?;
            }
            let hash = match evm::broadcast(&tor, Network::Sepolia, &tx, &key) {
                Ok(hash) => hash,
                Err(e) => return Ok(ShieldSplitSent { sent, stopped: Some(e.to_string()) }),
            };
            let _ =
                evm::nonce::record(&self.paths().account, Network::Sepolia, &from, tx.nonce, &hash);
            let link = format!("{}{}", Network::Sepolia.chain().explorer, to_hex(&hash));
            sent.push(PublicSent { hash: to_hex(&hash), link });
        }
        Ok(ShieldSplitSent { sent, stopped: None })
    }
}

/// An approval of the pool for some amount, exactly as this wallet writes one.
fn approved(data: &[u8]) -> Result<Option<Vec<u8>>, WalletError> {
    let amount =
        data.get(52..68).and_then(|b| <[u8; 16]>::try_from(b).ok()).map(u128::from_be_bytes);
    let Some(amount) = amount else { return Ok(None) };
    let ours = approve_calldata(amount)?;
    Ok((ours == data).then_some(ours))
}
