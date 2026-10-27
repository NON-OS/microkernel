// NONOS Operating System (AGPL-3.0-or-later)

//! A session on a radio whose chip runs CCMP-128 (`Crypto`): its keys put
//! in the chip's key table as mac80211 hands them to a driver's `set_key`,
//! the pairwise key first, then the group key with the IGTK; and every
//! frame the station marked Protected sent for the chip to encrypt. When
//! the chip refuses the join's pairwise key the station runs CCMP itself,
//! as mac80211 does when `set_key` fails; when it refuses a rekey's, the
//! session ends and the join runs again, since the chip still holds a key
//! the AP no longer uses. A group key it refuses stays with the station,
//! which opens any group frame the chip passes up undecrypted.

use super::state::Session;
use crate::key::Crypto;
use crate::softmac::radio::Radio;
use crate::station::LinkStation;

impl Session {
    /// Send `f`: a frame for the chip to protect by `send_keyed`, any other
    /// (in the clear, or sealed by the station) as it is.
    pub(super) fn put<R: Radio>(&self, r: &mut R, f: &[u8]) -> bool {
        let protected = f.get(1).is_some_and(|b| b & 0x40 != 0);
        if self.station.offloaded() && protected {
            r.send_keyed(f)
        } else {
            r.send(f)
        }
    }

    /// A rekey's keys into the chip: the group key at `group`, then the
    /// pairwise key when one was taken. False when the pairwise key was
    /// refused.
    pub(super) fn rekey_chip<R: Radio>(
        &self,
        r: &mut R,
        group: Option<u8>,
        pairwise: bool,
    ) -> bool {
        if !self.station.offloaded() {
            return true;
        }
        if let Some(g) = group.and_then(|id| self.station.chip_group(id)) {
            r.set_key(&g);
        }
        !pairwise || self.station.chip_pairwise().is_some_and(|p| r.set_key(&p))
    }
}

/// The join's keys into a radio that runs CCMP-128 in the chip, and the
/// station offloaded when the chip took the pairwise key.
pub(super) fn key_chip<R: Radio>(r: &mut R, station: &mut LinkStation) {
    let crypto = r.crypto();
    if crypto == Crypto::Software {
        return;
    }
    let Some(p) = station.chip_pairwise() else { return };
    if !r.set_key(&p) {
        return;
    }
    if let Some(g) = station.gtk_id().and_then(|id| station.chip_group(id)) {
        r.set_key(&g);
    }
    station.offload(crypto == Crypto::HostIv);
}
