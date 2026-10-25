// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The store open now, the locks counted against it, and the running proof's token.

use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use nox_shield_core::ffi::{CancelToken, Wallet};

pub(super) struct Open {
    pub(super) address: String,
    pub(super) dir: PathBuf,
    pub(super) wallet: Arc<Wallet>,
    pub(super) live: bool,
}

pub(super) static OPEN: Mutex<Option<Open>> = Mutex::new(None);
/// Counts locks. An open that began before the last lock does not become
/// the open store when it ends: the wallet locked while it ran.
pub(super) static LOCKS: AtomicU64 = AtomicU64::new(0);
pub(super) static CANCEL: Mutex<Option<Arc<CancelToken>>> = Mutex::new(None);

pub(crate) fn wallet() -> Result<Arc<Wallet>, String> {
    let open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?;
    open.as_ref()
        .map(|o| Arc::clone(&o.wallet))
        .ok_or_else(|| String::from("Open the shield from the wallet first."))
}
