// NONOS Operating System (AGPL-3.0-or-later)

//! The address each join runs under: one per network, from this link's
//! per-boot key (`nonos_wifi_core::address`).

use nonos_wifi_core::address::{AddressKey, KEY_LEN};
use nonos_wifi_core::wipe::wipe;

use super::buffers::RxBuffers;
use super::state::RtlLink;
use crate::fw::dma::DmaMem;
use crate::regs::Mmio;

/// Why a join has no address.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoAddress {
    /// No randomness for the key.
    NoKey,
    /// Still associated: a session keeps the address it began with.
    Associated,
}

impl<M: Mmio, D: DmaMem, RB: RxBuffers> RtlLink<M, D, RB> {
    /// Take the address for `ssid`: the key (drawn by `draw` the first time,
    /// then kept for the link's life) applied to the name, written to the
    /// MAC-ID registers and given to the station.
    pub fn readdress(
        &mut self,
        ssid: &[u8],
        draw: impl FnOnce() -> Option<[u8; KEY_LEN]>,
    ) -> Result<[u8; 6], NoAddress> {
        if self.address.is_none() {
            let mut key = draw().ok_or(NoAddress::NoKey)?;
            self.address = Some(AddressKey::new(&key));
            wipe(&mut key);
        }
        let mac = self.address.as_ref().ok_or(NoAddress::NoKey)?.for_network(ssid);
        if !self.station.readdress(mac) {
            return Err(NoAddress::Associated);
        }
        crate::mac::set_address(&self.mmio, &mac);
        Ok(mac)
    }
}
