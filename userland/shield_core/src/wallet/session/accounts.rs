//! The accounts of a session: how many, which is active, and changing either.
//! Adding one needs the seed, so the caller unlocks the vault again and the owner is confirmed.

use super::book::{Book, MAX_ACCOUNTS};
use super::slot::Slot;
use super::Session;
use crate::custody::Seed;
use crate::error::{StoreError, WalletError};
use crate::store::Row;
use crate::wallet::paths::Paths;

impl Session {
    pub(crate) fn at(&self, index: u32) -> Option<&Slot> {
        match index.checked_sub(1) {
            None => Some(&self.first),
            Some(i) => self.more.get(usize::try_from(i).ok()?),
        }
    }

    pub(super) fn slot(&self) -> &Slot {
        self.at(self.active).unwrap_or(&self.first)
    }

    pub fn count(&self) -> u32 {
        u32::try_from(self.more.len()).map_or(MAX_ACCOUNTS, |n| n.saturating_add(1))
    }

    pub fn active(&self) -> u32 {
        self.active
    }

    fn at_mut(&mut self, index: u32) -> Option<&mut Slot> {
        match index.checked_sub(1) {
            None => Some(&mut self.first),
            Some(i) => self.more.get_mut(usize::try_from(i).ok()?),
        }
    }

    pub(crate) fn record_to(&mut self, index: u32, row: Row) -> Result<(), StoreError> {
        let slot = self.at_mut(index).ok_or(StoreError::Io)?;
        slot.log.append(&row)?;
        slot.state.apply(row);
        Ok(())
    }

    /// Make account `index` the active one, and remember it.
    pub(crate) fn select(&mut self, paths: &Paths, index: u32) -> Result<(), WalletError> {
        if index >= self.count() {
            return Err(WalletError::NoSuchAccount);
        }
        Book { count: self.count(), active: index }.write(&paths.account)?;
        self.active = index;
        Ok(())
    }

    /// Derive the next account from `seed`, open its store, and make it active.
    pub(crate) fn add(&mut self, paths: &Paths, seed: &Seed) -> Result<u32, WalletError> {
        let index = self.count();
        if self.imported {
            return Err(WalletError::Unavailable);
        }
        if index >= MAX_ACCOUNTS {
            return Err(WalletError::NoSuchAccount);
        }
        self.more.push(Slot::open(paths, seed, index)?);
        self.next_public = super::open::next_public(seed, index.saturating_add(1));
        self.select(paths, index)?;
        Ok(index)
    }
}
