// NONOS Operating System (AGPL-3.0-or-later)

//! A link whose keys are in the chip (`Ccmp::Offload`): what the station
//! frames for the chip to encrypt, as mac80211 hands a hardware-keyed
//! driver a frame (`ieee80211_crypto_ccmp_encrypt` sets Protected; with
//! `IEEE80211_KEY_FLAG_GENERATE_IV` `ccmp_encrypt_skb` writes the CCMP
//! header and packet number, and without it leaves both to the chip; no
//! MIC either way). The keys as the chip takes them are `offload_keys`'.

use alloc::vec::Vec;

use super::{Ccmp, LinkStation};
use crate::dot11::ccmp::CCMP_HDR_LEN;
use crate::dot11::data::ccmp_header;
use crate::dot11::header::{header_len, FC_PROTECTED};

impl LinkStation {
    /// Whether the chip holds this link's keys and encrypts what it sends.
    pub fn offloaded(&self) -> bool {
        matches!(self.ccmp, Ccmp::Offload { .. })
    }

    /// The chip took the keys: a software CCMP-128 link becomes an offloaded
    /// one, keeping its key; any other mode is left as it is.
    pub fn offload(&mut self, host_iv: bool) {
        self.ccmp = match core::mem::replace(&mut self.ccmp, Ccmp::Hardware) {
            Ccmp::Software { tk } => Ccmp::Offload { tk, host_iv },
            other => other,
        };
    }

    /// `frame` marked Protected for the chip to encrypt; with `host_iv`, the
    /// CCMP header (key index 0) at the next packet number after its header.
    pub(super) fn chip_frame(&mut self, frame: &[u8], host_iv: bool) -> Option<Vec<u8>> {
        let fc = u16::from_le_bytes([*frame.first()?, *frame.get(1)?]);
        let hdr = header_len(fc);
        let mut out = Vec::with_capacity(frame.len() + CCMP_HDR_LEN);
        out.extend_from_slice(frame.get(..hdr)?);
        out[1] |= (FC_PROTECTED >> 8) as u8;
        if host_iv {
            self.pn = self.pn.wrapping_add(1);
            out.extend_from_slice(&ccmp_header(self.pn));
        }
        out.extend_from_slice(&frame[hdr..]);
        Some(out)
    }

    /// Whether `f` goes for the chip to seal: Protected on an offloaded link,
    /// and data, or a management frame where the chip writes the header; a
    /// station that writes it seals management frames itself, as mac80211
    /// does under `IEEE80211_KEY_FLAG_SW_MGMT_TX`.
    pub fn keyed(&self, f: &[u8]) -> bool {
        let Ccmp::Offload { host_iv, .. } = self.ccmp else { return false };
        let protected = f.get(1).is_some_and(|b| b & 0x40 != 0);
        protected && (f[0] >> 2 & 3 == 2 || !host_iv)
    }
}
