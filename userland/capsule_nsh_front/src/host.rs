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

//! The host's keys, as nsh.front reaches them: never the keys themselves.
//! nsh.attest made them on this machine at its first boot, keeps them
//! sealed to the TPM and signs with them; this side asks it for the public
//! keys once per connection and for a signature over each digest the
//! server signs. nsh.front holds Network, so it holds no host secret: a
//! process that took it over could ask for signatures while it runs, but
//! never carry the keys away.

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_libc::{mk_debug, mk_uptime_ms};
use nsh_front_wire::{
    ED_PUBLIC_LEN, ED_SIG_LEN, HDR_LEN, KEYS_LEN, ML_PUBLIC_LEN, OP_KEYS, OP_SIGN, SIG_LEN,
};
use nsh_server::{HostSig, HostSigner, Rand};

use crate::limits::RELAY_MS;
use crate::relay_said::{line, op_name, Fault};

// The wire's lengths are nsh/1's.
const _: () = assert!(ED_PUBLIC_LEN == nsh_wire::keys::ED_PUBLIC_LEN);
const _: () = assert!(ML_PUBLIC_LEN == nsh_wire::keys::ML_PUBLIC_LEN);
const _: () = assert!(SIG_LEN == nsh_wire::keys::HYBRID_SIG_LEN);

/// The host keys through nsh.attest.
pub struct HostBridge {
    port: u64,
    ed: [u8; ED_PUBLIC_LEN],
    ml: Box<[u8; ML_PUBLIC_LEN]>,
    resp: RefCell<Vec<u8>>,
}

/// Relay `op`, saying any fault on the serial console.
pub fn relay(port: u64, op: u16, body: &[u8], resp: &mut [u8]) -> Result<Vec<u8>, Fault> {
    let start = mk_uptime_ms();
    crate::relay::call(port, op, body, resp, RELAY_MS).inspect_err(|&f| {
        let waited = u64::try_from(mk_uptime_ms().wrapping_sub(start)).unwrap_or(0);
        let text = line(op_name(op), f, waited);
        let _ = mk_debug(text.as_ptr(), text.len());
    })
}

impl HostBridge {
    /// The host's public keys from nsh.attest on `port`, or why there are
    /// none.
    pub fn fetch(port: u32) -> Result<HostBridge, Fault> {
        let port = u64::from(port);
        let mut resp = vec![0u8; HDR_LEN + SIG_LEN];
        let keys = relay(port, OP_KEYS, &[], &mut resp)?;
        let keys: &[u8; KEYS_LEN] = keys.as_slice().try_into().map_err(|_| Fault::Unframed)?;
        let (ed, ml) = keys.split_at(ED_PUBLIC_LEN);
        let ed: [u8; ED_PUBLIC_LEN] = ed.try_into().map_err(|_| Fault::Unframed)?;
        let ml: [u8; ML_PUBLIC_LEN] = ml.try_into().map_err(|_| Fault::Unframed)?;
        Ok(HostBridge { port, ed, ml: Box::new(ml), resp: RefCell::new(resp) })
    }
}

impl HostSigner for HostBridge {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN] {
        &self.ed
    }

    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN] {
        &self.ml
    }

    /// nsh.attest signs with randomness of its own, so `_rand` is not drawn.
    fn sign(&self, digest: &[u8; 64], _rand: Rand<'_>) -> Option<HostSig> {
        let mut resp = self.resp.try_borrow_mut().ok()?;
        let sig = relay(self.port, OP_SIGN, digest, &mut resp).ok()?;
        let sig: &[u8; SIG_LEN] = sig.as_slice().try_into().ok()?;
        let (ed, ml) = sig.split_at(ED_SIG_LEN);
        Some(HostSig { ed: ed.try_into().ok()?, ml: Box::new(ml.try_into().ok()?) })
    }
}
