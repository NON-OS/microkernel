// NONOS Operating System (AGPL-3.0-or-later)

//! Before a connect leaves anything: a rejoin the person ended is refused,
//! and the address key and the join's randomness drawn. The network's own
//! address waits for its beacon, which says the security it is joined under.

use crate::server::join_wire::{ConnectRequest, ConnectResult, CODE_NO_ENTROPY};
use nonos_wifi_core::address::{Addresses, Security};
use nonos_wifi_core::link_history::CODE_LEFT;
use nonos_wifi_core::mlme::Entropy;
use nonos_wifi_core::vault::wipe;

use super::super::address::draw_key;
use super::super::entropy::draw_entropy;
use super::Join;

impl Join {
    /// The join's randomness, or the answer that ends it. The address key is
    /// drawn at the first join and kept.
    pub(super) fn ready(&mut self, req: &ConnectRequest<'_>) -> Result<Entropy, ConnectResult> {
        if req.rejoin && self.history.refuses_rejoin() {
            return Err(ConnectResult::code(CODE_LEFT));
        }
        let no_entropy = || ConnectResult::code(CODE_NO_ENTROPY);
        if self.address.is_none() {
            let mut key = draw_key().ok_or_else(no_entropy)?;
            self.address = Addresses::new(|k| {
                k.copy_from_slice(&key);
                true
            });
            wipe(&mut key);
        }
        draw_entropy().ok_or_else(no_entropy)
    }

    /// The address to join `ssid` under `kind`; none before `ready` drew the
    /// key.
    pub(super) fn address(&self, ssid: &[u8], kind: Security) -> Option<[u8; 6]> {
        self.address.as_ref().map(|k| k.for_network(ssid, kind))
    }
}
