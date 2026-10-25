// NONOS Operating System (AGPL-3.0-or-later)

//! The address each join runs under: one per network, a name under one
//! security, from this link's per-boot key (`nonos_wifi_core::address`).

use nonos_wifi_core::address::{Addresses, Security, KEY_LEN};
use nonos_wifi_core::vault::wipe;

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
    /// Ready the next join's address: refused while associated, and the key
    /// drawn by `draw` the first time, then kept for the link's life.
    pub fn address_ready(
        &mut self,
        draw: impl FnOnce() -> Option<[u8; KEY_LEN]>,
    ) -> Result<(), NoAddress> {
        if self.station.is_associated() {
            return Err(NoAddress::Associated);
        }
        if self.address.is_none() {
            let mut key = draw().ok_or(NoAddress::NoKey)?;
            self.address = Addresses::new(|k| {
                k.copy_from_slice(&key);
                true
            });
            wipe(&mut key);
        }
        Ok(())
    }

    /// Take the address for `ssid` joined under `kind`: the key applied to
    /// both, written to the MAC-ID registers and given to the station.
    pub fn readdress(&mut self, ssid: &[u8], kind: Security) -> Result<[u8; 6], NoAddress> {
        let mac = self.address.as_ref().ok_or(NoAddress::NoKey)?.for_network(ssid, kind);
        self.take_address(mac)?;
        Ok(mac)
    }

    /// Take `mac`, a network's or a hunt's one-off, for the station and the
    /// MAC-ID registers, so frames go out under it and answers to it are
    /// heard.
    pub fn take_address(&mut self, mac: [u8; 6]) -> Result<(), NoAddress> {
        if !self.station.readdress(mac) {
            return Err(NoAddress::Associated);
        }
        crate::mac::set_address(&self.mmio, &mac);
        Ok(())
    }
}
