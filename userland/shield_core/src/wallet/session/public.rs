//! The public account of any account, for a rewards link one account signs and another sends.

use super::Session;
use crate::evm::EvmAccount;

impl Session {
    pub(crate) fn evm_at(&self, index: u32) -> Option<&EvmAccount> {
        self.at(index).map(|slot| &slot.evm)
    }
}
