// NONOS Operating System (AGPL-3.0-or-later)

//! A link whose keys are in the chip (`Ccmp::Offload`): what the station
//! frames for the chip to encrypt, as mac80211 hands a hardware-keyed
//! driver a frame (`ieee80211_crypto_ccmp_encrypt` sets Protected; with
//! `IEEE80211_KEY_FLAG_GENERATE_IV` `ccmp_encrypt_skb` writes the CCMP
//! header and packet number, and without it leaves both to the chip; no
//! MIC either way), and the keys as the chip takes them, borrowed.

use alloc::vec::Vec;

use super::{Ccmp, LinkStation};
use crate::dot11::ccmp::CCMP_HDR_LEN;
use crate::dot11::data::ccmp_header;
use crate::dot11::header::{header_len, FC_PROTECTED};
use crate::key::ChipKey;
use crate::rsn::TemporalKey;

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

    /// The pairwise key, as a chip takes it.
    pub fn chip_pairwise(&self) -> Option<ChipKey<'_>> {
        match &self.ccmp {
            Ccmp::Software { tk } | Ccmp::Offload { tk, .. } => Some(ChipKey::Pairwise(tk)),
            _ => None,
        }
    }

    /// The CCMP-128 group key at `id` with, under management frame
    /// protection, the integrity group key; `None` for any other cipher,
    /// which the station keeps to itself.
    pub fn chip_group(&self, id: u8) -> Option<ChipKey<'_>> {
        let TemporalKey::Ccmp128(gtk) = self.group_keys.get(usize::from(id))?.as_ref()? else {
            return None;
        };
        let igtk = self.pmf.igtk.as_ref().map(|(k, i)| (*i as u8, k));
        Some(ChipKey::Group { id, gtk, igtk })
    }

    /// The index of the group key the AP last gave.
    pub fn gtk_id(&self) -> Option<u8> {
        self.supplicant.as_ref().map(|s| s.gtk_id())
    }
}
