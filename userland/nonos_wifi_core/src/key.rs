// NONOS Operating System (AGPL-3.0-or-later)
//! Key installation: the one place chips diverge on CCMP. Software chips stash
//! the key and encrypt per frame; hardware chips (Realtek's security CAM, and
//! Intel firmware's key table) program a slot and let the radio encrypt. The
//! shared supplicant installs keys through this trait after deriving them and
//! never names a hardware slot; the implementation chooses one.

pub trait KeyStore {
    /// Install the pairwise temporal key for `peer`. `key_id` is the 802.11 key
    /// index; the implementation picks the hardware slot. False if it could not
    /// be installed.
    fn install_ptk(&mut self, key: &[u8; 16], key_id: u8, peer: &[u8; 6]) -> bool;
    /// Install the group temporal key at `key_id`.
    fn install_gtk(&mut self, key: &[u8; 16], key_id: u8) -> bool;
    /// Remove a key: a pairwise key names its peer, a group key passes None.
    fn remove_key(&mut self, key_id: u8, peer: Option<&[u8; 6]>);
}

/// A 16-byte key on its way from a join or a rekey to the device: not Copy,
/// and wiped where it lies when dropped, which an assignment over it does in
/// place.
pub struct HeldKey(pub [u8; 16]);

impl Drop for HeldKey {
    fn drop(&mut self) {
        crate::vault::wipe(&mut self.0);
    }
}

/// Who runs CCMP-128 on a soft-MAC radio's link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crypto {
    /// The station seals and opens every frame itself.
    Software,
    /// The chip encrypts and decrypts under keys the station installs, and
    /// writes each sent frame's CCMP header and packet number itself
    /// (Linux mt76 sets no `IEEE80211_KEY_FLAG_GENERATE_IV`).
    ChipIv,
    /// As `ChipIv`, but the station writes the CCMP header with its own
    /// packet number and the chip encrypts behind it (Linux rtw89 sets
    /// `IEEE80211_KEY_FLAG_GENERATE_IV`).
    HostIv,
}

/// A CCMP-128 key on its way into a chip's key table, borrowed from the
/// station that holds it: the radio copies it into its command and wipes
/// that copy once the chip has it.
pub enum ChipKey<'a> {
    /// The pairwise key, index 0, of the access point.
    Pairwise(&'a [u8; 16]),
    /// A group key at its index and, under management frame protection, the
    /// integrity group key (index, key) that goes with it.
    Group { id: u8, gtk: &'a [u8; 16], igtk: Option<(u8, &'a [u8; 16])> },
}
