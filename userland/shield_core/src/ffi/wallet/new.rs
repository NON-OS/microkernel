//! Addressing a wallet in a private directory, its seed guarded by the platform keystore.

use super::Wallet;
use crate::custody::HardwareGuard;
use crate::error::WalletError;
use crate::wallet::Paths;
use std::sync::atomic::{AtomicU32, AtomicU64};
use std::sync::{Arc, Mutex};

impl Wallet {
    pub fn new(
        directory: String,
        guard: Arc<dyn HardwareGuard>,
    ) -> Result<Arc<Wallet>, WalletError> {
        Ok(Arc::new(Wallet {
            paths: Paths::under(directory),
            guard,
            session: Mutex::new(None),
            tor: Mutex::new(None),
            pending: Mutex::new(None),
            quoted: Mutex::new(None),
            batch: Mutex::new(None),
            settles: Mutex::new(None),
            reviews: AtomicU64::new(0),
            active: AtomicU32::new(0),
            searched: AtomicU32::new(0),
            stops: AtomicU64::new(0),
            read: Mutex::new(None),
        }))
    }
}
