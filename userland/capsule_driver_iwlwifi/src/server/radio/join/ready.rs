// NONOS Operating System (AGPL-3.0-or-later)

//! Before a connect leaves anything: a rejoin the person ended is refused,
//! the join's randomness is drawn, and the network's own address taken.

use crate::server::join_wire::{ConnectRequest, ConnectResult, CODE_NO_ENTROPY};
use nonos_wifi_core::address::AddressKey;
use nonos_wifi_core::link_history::CODE_LEFT;
use nonos_wifi_core::mlme::Entropy;
use nonos_wifi_core::wipe::wipe;

use super::super::address::draw_key;
use super::super::entropy::draw_entropy;
use super::Join;

impl Join {
    /// The address the join runs under and its randomness, or the answer
    /// that ends it. The key is drawn at the first join and kept.
    pub(super) fn ready(
        &mut self,
        req: &ConnectRequest<'_>,
    ) -> Result<([u8; 6], Entropy), ConnectResult> {
        if req.rejoin && self.history.refuses_rejoin() {
            return Err(ConnectResult::code(CODE_LEFT));
        }
        let no_entropy = || ConnectResult::code(CODE_NO_ENTROPY);
        if self.address.is_none() {
            let mut key = draw_key().ok_or_else(no_entropy)?;
            self.address = Some(AddressKey::new(&key));
            wipe(&mut key);
        }
        let addr = self.address.as_ref().ok_or_else(no_entropy)?.for_network(req.ssid);
        Ok((addr, draw_entropy().ok_or_else(no_entropy)?))
    }
}
