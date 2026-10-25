// NONOS Operating System (AGPL-3.0-or-later)
//! The store this crate exports, made the way the keyring makes its own: an
//! empty store whose first key takes id 1.

use super::types::Store;

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
