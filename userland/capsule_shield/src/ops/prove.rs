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

//! The wallet made ready to prove, and the token a proof runs under: its progress, its stop.

use std::sync::Arc;

use nox_shield_core::ffi::{CancelToken, Wallet};
use shield_wire::Values;

use super::held::{CANCEL, OPEN};
use super::{refuse, Answer};

/// The open wallet, made ready to prove: a thread for the prover, and the
/// shipped periodic cache in its folder.
pub(super) fn prover() -> Result<Arc<Wallet>, String> {
    if crate::pool::workers() == 0 {
        return refuse("No thread could be started for the prover.");
    }
    /* The folder and the wallet are taken and the lock let go before the cache is written:
     * an 8 MiB write under OPEN would hold the service loop past the wallet's timeout. */
    let (dir, wallet) = {
        let open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?;
        let o =
            open.as_ref().ok_or_else(|| String::from("Open the shield from the wallet first."))?;
        (o.dir.clone(), Arc::clone(&o.wallet))
    };
    crate::periodic::seed(&dir);
    Ok(wallet)
}

pub(super) fn cancel_token() -> Result<Arc<CancelToken>, String> {
    let token = CancelToken::new();
    *CANCEL.lock().map_err(|_| String::from("the shield is busy"))? = Some(Arc::clone(&token));
    Ok(token)
}

/// The proof is over: its token goes, so the next spend never shows its progress.
pub(super) fn proved() {
    if let Ok(mut c) = CANCEL.lock() {
        *c = None;
    }
}

/// How far the running spend's proof has come: the last phase the prover
/// finished, by name, and the fraction of the proof done then.
pub fn progress() -> Option<(String, f32)> {
    CANCEL.lock().ok()?.as_ref()?.progress()
}

pub fn cancel() -> Answer {
    if let Some(t) = CANCEL.lock().map_err(|_| String::from("the shield is busy"))?.as_ref() {
        t.cancel();
    }
    Ok(Values::new())
}
